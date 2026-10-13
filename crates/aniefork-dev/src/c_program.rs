//! The loader a C project's page runs its program with: `program.js`, served beside the page
//! as `spreadsheet.js` is ([`crate::bridge`]). It is not a file of the project.
//!
//! A C project is built as one WebAssembly program ([`crate::c`]). A WebAssembly program can do
//! nothing by itself: it asks whoever runs it for everything. The loader hands it the little of
//! a system a C program asks for (a clock, a place to print, random numbers, its arguments, and
//! files kept in memory) and nothing else: no way to the spreadsheet, to the network or to the
//! computer's files. What the program is to do with the workbook, the project's page does for
//! it through `spreadsheet`, where every call is checked against what the project was granted.
//! That is what holds a C project to what it asked for.
//!
//! This is JavaScript, as text inside Rust source: the fork's fourth exception to upstream's
//! "Rust only" rule, after the panel's `spreadsheet.js`, the Svelte showcase and the Development
//! window's page. A page is what runs a WebAssembly program, and a page is written in
//! JavaScript: there is no Rust that could stand in its place. The repository holds no
//! JavaScript file, and building GridCraft builds none.

/// `program.js`, as it is served.
pub const LOADER: &str = r#"// GridCraft's loader for a C project's program. GridCraft serves it beside the project's page
// as program.js, as it serves spreadsheet.js; it is not a file of the project.
//
// A C project is built as one WebAssembly program (program.wasm, served beside the page too:
// the one built from the project's files as you trust them now). A WebAssembly program can do
// nothing by itself: it asks whoever runs it for everything. This gives it the little of a
// system a C program asks for, and nothing more:
//
//   a clock, a place to print, random numbers, its arguments,
//   and files that are kept in memory: the ones the page hands it, and what it writes itself
//   (gone when the page is closed).
//
// It gives it no way to the spreadsheet, to the network or to the computer's files. What the
// program is to do with the workbook, the page does for it through `spreadsheet`, where every
// call is checked against what the project was granted.
//
//   const program = await CProgram.load({
//     files: { "data.bin": bytes },          // optional: files the program can open
//     args: ["program", "-flag"],            // optional: what main() is handed
//     print: (line) => console.log(line),    // optional: where what it prints goes
//   });
//   program.run();                           // a program with a main: runs it to its end
//   program.exports.step();                  // or call the functions it exports
//   program.string(pointer)                  // a C string of the program's, as text
//   program.bytes(pointer, length)           // a look at the program's memory
//   program.files.read("save.dat")           // what it wrote
//   program.ended()                          // why it ended, or null while it can still run

"use strict";

const CProgram = (() => {
  // The answers a WebAssembly program is given by the system it runs on (WASI's numbers).
  const OK = 0, BADF = 8, EXIST = 20, INVAL = 28, ISDIR = 31, NOENT = 44, NOSYS = 52, PERM = 63;

  class Ended extends Error {}

  /** The parts of a path, without the empty ones, "." and what ".." takes back. */
  function parts(path) {
    const kept = [];
    for (const part of String(path).split("/")) {
      if (part === "" || part === ".") continue;
      if (part === "..") kept.pop();
      else kept.push(part);
    }
    return kept.join("/");
  }

  /**
   * Starts a program from its bytes. `files` are the files it can open, by name; `args` what its
   * main is handed; `now()` the clock, in milliseconds; `print(line)` is handed each line it prints.
   */
  function start(wasm, { files = {}, args = ["program"], now = () => performance.now(), print = (line) => console.log(line) } = {}) {
    const kept = new Map();
    for (const [name, data] of Object.entries(files)) {
      const held = data instanceof Uint8Array ? data : new Uint8Array(data);
      kept.set(parts(name), { data: held, size: held.length, fixed: true });
    }
    const folders = new Set([""]);
    const open = new Map();
    const encoder = new TextEncoder();
    const decoder = new TextDecoder();
    const handed = args.map((arg) => encoder.encode(String(arg)));
    let nextFd = 4;
    let memory = null;
    let printed = "";
    let ended = null;
    let endedWith = 0;

    const view = () => new DataView(memory.buffer);
    const bytes = () => new Uint8Array(memory.buffer);
    const path = (at, length) => parts(decoder.decode(bytes().subarray(at, at + length)));

    function say(chunk) {
      printed += chunk;
      let line;
      while ((line = printed.indexOf("\n")) >= 0) {
        print(printed.slice(0, line));
        printed = printed.slice(line + 1);
      }
    }

    /** Each of the pieces a read or a write is made of: where it is, and how long. */
    function* pieces(at, count) {
      const v = view();
      for (let i = 0; i < count; i++) yield [v.getUint32(at + i * 8, true), v.getUint32(at + i * 8 + 4, true)];
    }

    /** What is known of a file or a folder, as the program is told it. */
    function describe(at, folder, size) {
      const v = view();
      for (let i = 0; i < 64; i += 8) v.setBigUint64(at + i, 0n, true);
      v.setUint8(at + 16, folder ? 3 : 4);
      v.setBigUint64(at + 24, 1n, true);
      v.setBigUint64(at + 32, BigInt(size), true);
      return OK;
    }

    const system = {
      args_sizes_get(countAt, sizeAt) {
        view().setUint32(countAt, handed.length, true);
        view().setUint32(sizeAt, handed.reduce((sum, arg) => sum + arg.length + 1, 0), true);
        return OK;
      },
      args_get(listAt, textAt) {
        for (const arg of handed) {
          view().setUint32(listAt, textAt, true);
          bytes().set(arg, textAt);
          bytes()[textAt + arg.length] = 0;
          listAt += 4;
          textAt += arg.length + 1;
        }
        return OK;
      },
      environ_sizes_get(countAt, sizeAt) {
        view().setUint32(countAt, 0, true);
        view().setUint32(sizeAt, 0, true);
        return OK;
      },
      environ_get() {
        return OK;
      },
      clock_time_get(id, precision, at) {
        const ms = id === 0 ? Date.now() : now();
        view().setBigUint64(at, BigInt(Math.round(ms * 1e6)), true);
        return OK;
      },
      clock_res_get(id, at) {
        view().setBigUint64(at, 1000n, true);
        return OK;
      },
      random_get(at, length) {
        const into = bytes().subarray(at, at + length);
        // Asked for at most 65536 at a time.
        for (let done = 0; done < length; done += 65536) crypto.getRandomValues(into.subarray(done, Math.min(length, done + 65536)));
        return OK;
      },
      sched_yield() {
        return OK;
      },
      proc_exit(code) {
        throw new Ended(String(code));
      },
      fd_prestat_get(fd, at) {
        if (fd !== 3) return BADF;
        view().setUint8(at, 0);
        view().setUint32(at + 4, 1, true);
        return OK;
      },
      fd_prestat_dir_name(fd, at, length) {
        if (fd !== 3 || length < 1) return BADF;
        bytes()[at] = 47; // "/"
        return OK;
      },
      fd_fdstat_get(fd, at) {
        const kind = fd < 3 ? 2 : fd === 3 || open.get(fd)?.folder ? 3 : open.has(fd) ? 4 : 0;
        if (!kind) return BADF;
        const v = view();
        v.setBigUint64(at, 0n, true);
        v.setUint8(at, kind);
        v.setBigUint64(at + 8, 0xffffffffffffffffn, true);
        v.setBigUint64(at + 16, 0xffffffffffffffffn, true);
        return OK;
      },
      fd_fdstat_set_flags() {
        return OK;
      },
      fd_filestat_get(fd, at) {
        const held = open.get(fd);
        if (fd === 3 || held?.folder) return describe(at, true, 0);
        return held?.file ? describe(at, false, held.file.size) : BADF;
      },
      path_filestat_get(dir, flags, pathAt, pathLength, at) {
        const name = path(pathAt, pathLength);
        if (folders.has(name)) return describe(at, true, 0);
        const file = kept.get(name);
        return file ? describe(at, false, file.size) : NOENT;
      },
      path_open(dir, dirFlags, pathAt, pathLength, flags, rights, inherited, fdFlags, fdAt) {
        const name = path(pathAt, pathLength);
        const [create, folder, alone, empty] = [flags & 1, flags & 2, flags & 4, flags & 8];
        if (folders.has(name)) {
          if (!folder && (create || empty)) return ISDIR;
          open.set(nextFd, { folder: true });
        } else {
          if (folder) return NOENT;
          let file = kept.get(name);
          if (file && create && alone) return EXIST;
          // A file the page handed over is read, never written over (64: the leave to write).
          if (file?.fixed && (empty || (BigInt(rights) & 64n) !== 0n)) return PERM;
          if (!file) {
            // A new file goes in a folder that is there.
            if (!create || !folders.has(name.includes("/") ? name.slice(0, name.lastIndexOf("/")) : "")) return NOENT;
            file = { data: new Uint8Array(0), size: 0 };
            kept.set(name, file);
          }
          if (empty) file.size = 0;
          open.set(nextFd, { file, at: 0, append: !!(fdFlags & 1) });
        }
        view().setUint32(fdAt, nextFd++, true);
        return OK;
      },
      fd_close(fd) {
        return open.delete(fd) || fd <= 3 ? OK : BADF;
      },
      fd_read(fd, at, count, doneAt) {
        const held = open.get(fd);
        if (!held?.file) return fd === 0 ? (view().setUint32(doneAt, 0, true), OK) : BADF;
        let done = 0;
        for (const [to, length] of pieces(at, count)) {
          const take = Math.max(0, Math.min(length, held.file.size - held.at));
          bytes().set(held.file.data.subarray(held.at, held.at + take), to);
          held.at += take;
          done += take;
          if (take < length) break;
        }
        view().setUint32(doneAt, done, true);
        return OK;
      },
      fd_write(fd, at, count, doneAt) {
        const held = open.get(fd);
        if (fd !== 1 && fd !== 2 && !held?.file) return BADF;
        let done = 0;
        for (const [from, length] of pieces(at, count)) {
          const chunk = bytes().subarray(from, from + length);
          if (held) {
            const file = held.file;
            if (file.fixed) return PERM;
            if (held.append) held.at = file.size;
            if (held.at + length > file.data.length) {
              const larger = new Uint8Array(Math.max(held.at + length, file.data.length * 2, 4096));
              larger.set(file.data.subarray(0, file.size));
              file.data = larger;
            }
            file.data.set(chunk, held.at);
            held.at += length;
            file.size = Math.max(file.size, held.at);
          } else {
            say(decoder.decode(chunk));
          }
          done += length;
        }
        view().setUint32(doneAt, done, true);
        return OK;
      },
      fd_seek(fd, offset, whence, at) {
        const held = open.get(fd);
        if (!held?.file) return BADF;
        const from = whence === 0 ? 0 : whence === 1 ? held.at : whence === 2 ? held.file.size : -1;
        const to = from + Number(offset);
        if (from < 0 || to < 0) return INVAL;
        held.at = to;
        view().setBigUint64(at, BigInt(to), true);
        return OK;
      },
      fd_tell(fd, at) {
        const held = open.get(fd);
        if (!held?.file) return BADF;
        view().setBigUint64(at, BigInt(held.at), true);
        return OK;
      },
      path_create_directory(dir, pathAt, pathLength) {
        folders.add(path(pathAt, pathLength));
        return OK;
      },
      path_remove_directory(dir, pathAt, pathLength) {
        return folders.delete(path(pathAt, pathLength)) ? OK : NOENT;
      },
      path_unlink_file(dir, pathAt, pathLength) {
        const name = path(pathAt, pathLength);
        if (kept.get(name)?.fixed) return PERM;
        return kept.delete(name) ? OK : NOENT;
      },
      path_rename(dir, fromAt, fromLength, toDir, toAt, toLength) {
        const [from, to] = [path(fromAt, fromLength), path(toAt, toLength)];
        const file = kept.get(from);
        if (!file) return NOENT;
        if (file.fixed || kept.get(to)?.fixed) return PERM;
        kept.delete(from);
        kept.set(to, file);
        return OK;
      },
    };

    const module = new WebAssembly.Module(wasm);
    // Anything else the program was built to ask for is answered "there is no such thing".
    const given = {};
    for (const asked of WebAssembly.Module.imports(module)) {
      if (asked.kind !== "function") throw new Error(`The program asks for ${asked.module}.${asked.name}, which is not something a page can hand it.`);
      given[asked.module] ??= {};
      given[asked.module][asked.name] = (asked.module === "wasi_snapshot_preview1" && system[asked.name]) || (() => NOSYS);
    }
    const exports = new WebAssembly.Instance(module, given).exports;
    memory = exports.memory;
    if (!(memory instanceof WebAssembly.Memory)) throw new Error("The program has no memory of its own to look at.");

    /** Runs something of the program's. Once it has ended, nothing of it runs again. */
    function guarded(call, otherwise) {
      if (ended) return otherwise;
      try {
        return call();
      } catch (error) {
        if (printed) say("\n");
        if (error instanceof Ended) {
          endedWith = Number(error.message);
          ended = `The program ended (${endedWith}).`;
          return endedWith;
        }
        endedWith = 1;
        ended = `The program stopped: ${error.message || error}`;
        print(ended);
        return otherwise;
      }
    }

    // A program whose page calls what it exports sets itself up first.
    if (typeof exports._initialize === "function") guarded(() => exports._initialize());

    // The program's own functions, each stopped from running once the program has ended.
    const functions = {};
    for (const [name, value] of Object.entries(exports)) {
      if (typeof value === "function" && name !== "_start" && name !== "_initialize") functions[name] = (...given) => guarded(() => value(...given));
    }

    return {
      exports: functions,
      memory,
      /** Why the program ended, or null while it can still run. */
      ended: () => ended,
      /** Runs a program that has a main to its end, and gives what it ended with (0: well). Once. */
      run() {
        if (typeof exports._start !== "function") throw new Error("This program has no main to run: call the functions it exports.");
        if (ended) return endedWith;
        const code = guarded(() => (exports._start(), 0), 1);
        if (!ended) ended = "The program ended (0).";
        return code;
      },
      /** The text at a pointer of the program's, up to its closing zero. */
      string(at) {
        const all = bytes();
        let end = at;
        while (end < all.length && all[end] !== 0) end++;
        return decoder.decode(all.subarray(at, end));
      },
      /** A look at the program's memory: good until the program next runs. */
      bytes: (at, length) => new Uint8Array(memory.buffer, at, length),
      files: {
        /** A copy of a file kept in memory, or null. */
        read(name) {
          const file = kept.get(parts(name));
          return file ? file.data.slice(0, file.size) : null;
        },
        /** Puts a file where the program can open it and write over it. */
        write(name, data) {
          const held = data instanceof Uint8Array ? data.slice() : new Uint8Array(data);
          kept.set(parts(name), { data: held, size: held.length });
        },
        /** Every file kept, and its size. */
        list: () => [...kept.entries()].map(([name, file]) => [name, file.size]),
      },
    };
  }

  /**
   * Loads the project's program (program.wasm, beside the page) and starts it. Takes what
   * `start` takes.
   */
  async function load(options = {}) {
    const answer = await fetch(options.url || "program.wasm");
    if (answer.status === 404) {
      throw new Error("This project has not been built from the files it holds now. Build it in GridCraft: Developer, Development, the project, Build.");
    }
    if (!answer.ok) {
      // GridCraft says why in a sentence: the project changed since it was trusted, or is not trusted.
      const why = (await answer.text()).trim();
      throw new Error(why && why.length <= 300 ? why : `The program could not be loaded (${answer.status}).`);
    }
    return start(new Uint8Array(await answer.arrayBuffer()), options);
  }

  return { load, start };
})();

if (typeof module !== "undefined") module.exports = CProgram;
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_loader_hands_a_program_a_small_system_and_no_way_out_of_the_page() {
        // What a page is given, by name.
        for part in ["const CProgram", "async function load", "function start", "program.wasm", "wasi_snapshot_preview1"] {
            assert!(LOADER.contains(part), "{part}");
        }
        // What a program asks of a system, and is given: a clock, printing, its arguments, files in memory.
        for call in ["clock_time_get", "fd_write", "fd_read", "path_open", "args_get", "random_get", "proc_exit"] {
            assert!(LOADER.contains(&format!("{call}(")), "{call}");
        }
        // Nothing in it reaches further: it loads the one program, and never names the spreadsheet,
        // another address, or a way to run text as script.
        assert_eq!(LOADER.matches("fetch(").count(), 1, "the program, and nothing else");
        for reach in [
            "spreadsheet.run",
            "spreadsheet.batch",
            "spreadsheet.write",
            "spreadsheet.read",
            "spreadsheet.log",
            "XMLHttpRequest",
            "WebSocket",
            "eval(",
            "new Function",
            "importScripts",
            "document.",
            "localStorage",
        ] {
            assert!(!LOADER.contains(reach), "{reach}");
        }
        // What a program asks for that the loader doesn't know is answered "there is no such thing".
        assert!(LOADER.contains("(() => NOSYS)"));
        // It is plain text a browser reads as it is: nothing a build step would have to make of it.
        assert!(LOADER.is_ascii() && !LOADER.contains('\r') && !LOADER.contains("import ") && !LOADER.contains("require("));
    }
}
