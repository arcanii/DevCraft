"""Synthetic workbooks and command scripts for memory measurements (our own data, nothing from Excel).

Usage: python3 -I make_books.py <out dir>. Writes, as CSV (GridCraft opens CSV; convert to .xlsx with
`gridcraft-cli convert` where a measurement wants it):
  values-200k.csv   200,000 rows x 10 columns of numbers (2 M cells)
  text-200k.csv     200,000 rows x 5 columns of words from a small list (1 M text cells)
  line-latin.csv, line-cjk.csv, line-emoji.csv, line-scripts.csv   one cell each, for fonts
  noise.png         a 2600 x 2600 picture of noise, about 19 MB, that doesn't compress
and command scripts for `gridcraft-cli run --script`, each step one undo step:
  edits-N.jsonl          N cell edits (N = 1, 10, 50, 200)
  sorts-N.jsonl          N sorts of A1:J200001, alternately up and down (N = 1, 5, 20)
  bolds-N.jsonl          N bold toggles over A1:J200001 (N = 1, 5, 20)
  picture-edits-N.jsonl  insert noise.png at B2, then N cell edits on the same sheet (N = 0, 10, 50)
  formulas-100k.jsonl    a running total in D2 filled down to D100001 (slow: see the record)
"""
import json, os, random, struct, sys, zlib

out = sys.argv[1]
os.makedirs(out, exist_ok=True)
random.seed(7)
WORDS = ("garden river lantern quiet harbour meadow copper window orchard pebble signal "
         "thunder velvet compass ember harvest willow beacon canvas marble journey").split()

def write(name, text):
    with open(os.path.join(out, name), "w", encoding="utf-8", newline="\n") as f:
        f.write(text)

def cmd(command, params):
    return json.dumps({"command": command, "params": params})

def script(name, lines):
    write(name, "\n".join(lines) + "\n")

rows = [",".join(f"c{c}" for c in range(10))]
for r in range(200_000):
    rows.append(",".join(str(random.randrange(1_000_000) / 100) for _ in range(10)))
write("values-200k.csv", "\n".join(rows) + "\n")

rows = [",".join(f"t{c}" for c in range(5))]
for r in range(200_000):
    rows.append(",".join(random.choice(WORDS) for _ in range(5)))
write("text-200k.csv", "\n".join(rows) + "\n")

write("line-latin.csv", "Quiet harbour\n")
write("line-cjk.csv", "你好世界\n")
write("line-emoji.csv", "\U0001F642\n")
write("line-scripts.csv", "中文 \U0001F642 ไทย हिन्दी العربية\n")

w = h = 2600
raw = b"".join(b"\x00" + os.urandom(w * 3) for _ in range(h))
def chunk(t, d):
    c = struct.pack(">I", len(d)) + t + d
    return c + struct.pack(">I", zlib.crc32(t + d) & 0xFFFFFFFF)
png = (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0))
       + chunk(b"IDAT", zlib.compress(raw, 1)) + chunk(b"IEND", b""))
picture = os.path.join(out, "noise.png")
with open(picture, "wb") as f:
    f.write(png)

for n in (1, 10, 50, 200):
    script(f"edits-{n}.jsonl", [cmd("cell.set", {"cell": f"L{i + 2}", "input": str(i)}) for i in range(n)])
for n in (1, 5, 20):
    script(f"sorts-{n}.jsonl", [cmd("data.sortAscending" if i % 2 == 0 else "data.sortDescending",
                                    {"range": "A1:J200001", "column": "B"}) for i in range(n)])
    script(f"bolds-{n}.jsonl", [cmd("home.bold", {"range": "A1:J200001", "on": i % 2 == 0}) for i in range(n)])
for n in (0, 10, 50):
    script(f"picture-edits-{n}.jsonl", [cmd("insert.picture", {"path": picture, "at": "B2"})]
           + [cmd("cell.set", {"cell": f"A{i + 1}", "input": str(i)}) for i in range(n)])
script("formulas-100k.jsonl", [
    cmd("cell.set", {"cell": "D2", "input": "=B2*C2+SUM($A$2:A2)"}),
    cmd("edit.fillDown", {"range": "D2:D100001"}),
])
