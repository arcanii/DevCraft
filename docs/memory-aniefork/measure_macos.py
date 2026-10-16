"""Run a program and report its memory on macOS: the kernel's peak and the final physical footprint
(what Activity Monitor calls "Memory"; on Apple silicon it includes the process's GPU memory) and
resident size, sampled every 50 ms, plus a coarse timeline. The macOS counterpart of measure.ps1.

Usage: python3 -I measure_macos.py <label> [NAME=VALUE ...] -- <program> [args ...]

The peak is the kernel's lifetime maximum (rusage_info_v4.ri_lifetime_max_phys_footprint), read
after the program exits and before it is reaped, so a short run cannot miss it.
"""
import ctypes, os, subprocess, sys, tempfile, time

RUSAGE_INFO_V4 = 4
RESIDENT, FOOTPRINT, LIFETIME_MAX = 2 + 6, 2 + 7, 2 + 28  # u64 offsets past the 16-byte uuid
libproc = ctypes.CDLL("/usr/lib/libproc.dylib", use_errno=True)

def rusage(pid):
    buf = (ctypes.c_uint64 * 64)()
    if libproc.proc_pid_rusage(pid, RUSAGE_INFO_V4, ctypes.byref(buf)) != 0:
        return None
    return buf

def mb(b):
    return round(b / (1 << 20))

args = sys.argv[1:]
if "--" not in args or args.index("--") < 1:
    sys.exit(__doc__)
split = args.index("--")
label, env_pairs, cmd = args[0], args[1:split], args[split + 1:]
env = dict(os.environ)
for pair in env_pairs:
    k, _, v = pair.partition("=")
    env[k] = v

start = time.monotonic()
errfile = tempfile.TemporaryFile()
p = subprocess.Popen(cmd, env=env, stdout=subprocess.DEVNULL, stderr=errfile)

def exited():
    # WNOWAIT leaves the process a zombie, so its peak can still be read.
    if hasattr(os, "waitid"):
        return os.waitid(os.P_PID, p.pid, os.WEXITED | os.WNOHANG | os.WNOWAIT) is not None
    return p.poll() is not None

max_fp = last_fp = last_rss = 0
timeline, next_mark = [], 0.0
while not exited():
    r = rusage(p.pid)
    if r is not None and r[FOOTPRINT] > 0:
        last_fp, last_rss = r[FOOTPRINT], r[RESIDENT]
        max_fp = max(max_fp, last_fp)
        t = time.monotonic() - start
        if t >= next_mark:
            timeline.append(f"{t:.1f}s:{mb(last_fp)}")
            next_mark += 1.0
    time.sleep(0.05)

elapsed = time.monotonic() - start
r = rusage(p.pid)  # a zombie until wait() reaps it
peak = r[LIFETIME_MAX] if r is not None else 0
p.wait()
errfile.seek(0)
err = errfile.read()
peak_note = f"peak {mb(peak)} MB" if peak else f"peak {mb(max_fp)} MB (sampled)"
print(f"{label:<34} exit {p.returncode}  {peak_note}  sampled peak {mb(max_fp)} MB  "
      f"last {mb(last_fp)} MB  last RSS {mb(last_rss)} MB  ({elapsed:.1f} s)")
print("  timeline (s:MB footprint): " + " ".join(timeline))
err = err.decode("utf-8", "replace").strip()
if p.returncode != 0 and err:
    print("  stderr: " + err[:300])
