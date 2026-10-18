//! The loader a C project's page runs its program with: `program.js`, served beside the page
//! as `spreadsheet.js` is ([`crate::bridge`]). It is not a file of the project.
//!
//! A C project is built as one WebAssembly program ([`crate::c`]). A WebAssembly program can do
//! nothing by itself: it asks whoever runs it for everything. The loader hands it the little of
//! a system a C program asks for (a clock, a place to print, random numbers, its arguments, and
//! files kept in memory), and, when the page names a canvas, OpenGL ES 3.0 to draw on that
//! canvas with (the owner, 2026-10-14 and 2026-10-19; the header is [`crate::c::GL_HEADER`]). And
//! nothing else: no way to the spreadsheet or to the computer's files, and none to the network
//! but GridCraft's, which carries the packets of a program whose project names where it would
//! reach, once the person has turned its network on ([`crate::net`]). What the program is to do
//! with the workbook, the project's page does for it through `spreadsheet`, where every call is
//! checked against what the project was granted. That is what holds a C project to what it
//! asked for.
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
// It gives it no way to the spreadsheet or to the computer's files, and none to the network but
// GridCraft's: for a project that names where it would reach, once the person has turned its
// network on, GridCraft carries its packets there and takes in the answers (<gridcraft/net.h>).
// What the program is to do with the workbook, the page does for it through `spreadsheet`,
// where every call is checked against what the project was granted.
//
//   const program = await CProgram.load({
//     files: { "data.bin": bytes },          // optional: files the program can open
//     args: ["program", "-flag"],            // optional: what main() is handed
//     print: (line) => console.log(line),    // optional: where what it prints goes
//     canvas,                                // optional: a canvas it draws on with OpenGL ES
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
   * main is handed; `now()` the clock, in milliseconds; `print(line)` is handed each line it prints;
   * `canvas` is where it draws, when it draws with OpenGL ES; `network` is GridCraft's address
   * that carries its packets (load names it: the program's own address, with `net` for its name).
   */
  function start(wasm, { files = {}, args = ["program"], now = () => performance.now(), print = (line) => console.log(line), canvas = null, network = null } = {}) {
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
        // What is printed goes to a terminal, as the program sees it: it can't be sought in or
        // asked where it is. So the program's C library sends each line as it is printed, and a
        // program that keeps running (its page calls what it exports) holds none back.
        const rights = fd < 3 ? 0xffffffffffffffdbn : 0xffffffffffffffffn;
        v.setBigUint64(at + 8, rights, true);
        v.setBigUint64(at + 16, rights, true);
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

    // OpenGL ES 3.0, for a program whose page names a canvas to draw on: each call the program
    // makes is answered with that canvas's WebGL 2, which is OpenGL ES 3.0 as a page has it.
    // What OpenGL names by number (a buffer, a shader, a texture) is kept here under that
    // number; 0 is none. It draws on the one canvas and reaches nothing else of the page.
    const drawing = canvas ? canvas.getContext("webgl2", { alpha: false, antialias: true }) : null;
    // The extensions of WebGL turned on for the program: those of these the canvas offers, and no
    // others. Each is told the program under the name OpenGL gives it (glGetString); ETC2, which
    // OpenGL ES 3.0 has of its own, is turned on where the canvas has it and named nowhere.
    const glExtensions = [];
    if (drawing) {
      for (const [webgl, named] of [
        ["EXT_color_buffer_float", "GL_EXT_color_buffer_float"],
        ["EXT_color_buffer_half_float", "GL_EXT_color_buffer_half_float"],
        ["EXT_float_blend", "GL_EXT_float_blend"],
        ["OES_texture_float_linear", "GL_OES_texture_float_linear"],
        ["EXT_texture_filter_anisotropic", "GL_EXT_texture_filter_anisotropic"],
        ["EXT_texture_norm16", "GL_EXT_texture_norm16"],
        ["EXT_render_snorm", "GL_EXT_render_snorm"],
        ["EXT_texture_mirror_clamp_to_edge", "GL_EXT_texture_mirror_clamp_to_edge"],
        ["EXT_depth_clamp", "GL_EXT_depth_clamp"],
        ["WEBGL_compressed_texture_s3tc", "GL_EXT_texture_compression_s3tc"],
        ["WEBGL_compressed_texture_s3tc_srgb", "GL_EXT_texture_compression_s3tc_srgb"],
        ["EXT_texture_compression_rgtc", "GL_EXT_texture_compression_rgtc"],
        ["EXT_texture_compression_bptc", "GL_EXT_texture_compression_bptc"],
        ["WEBGL_compressed_texture_etc", null],
      ]) {
        if (drawing.getExtension(webgl) && named) glExtensions.push(named);
      }
    }
    const made = {
      buffer: [null], shader: [null], program: [null], texture: [null], array: [null], place: [null],
      framebuffer: [null], renderbuffer: [null], sampler: [null], query: [null], feedback: [null], sync: [null],
    };
    const keep = (kind, object) => (object ? made[kind].push(object) - 1 : 0);
    // An error the loader found itself (a call WebGL has no way to make), told before WebGL's own.
    let glOwnError = 0;
    const glFail = (error) => {
      if (!glOwnError) glOwnError = error;
    };
    const glText = (at) => {
      const all = bytes();
      let end = at;
      while (end < all.length && all[end] !== 0) end++;
      return decoder.decode(all.subarray(at, end));
    };
    /** So many texts, from a list of pointers to them. */
    const glTexts = (at, count) => Array.from({ length: Math.max(0, count) }, (_, i) => glText(view().getUint32(at + i * 4, true)));
    /** So many numbers of the program's, as the kind of array WebGL takes them in: a look at its memory, or a copy where they are not aligned for it. */
    const glArray = (Kind, at, count) => {
      const size = Math.max(0, count) * Kind.BYTES_PER_ELEMENT;
      if (!at || size === 0) return new Kind(0);
      return at % Kind.BYTES_PER_ELEMENT === 0 ? new Kind(memory.buffer, at, count) : new Kind(memory.buffer.slice(at, at + size));
    };
    const glFloats = (at, count) => glArray(Float32Array, at, count);
    /** Numbers written into the program's memory, as the kind it asked for. */
    const glPut = {
      i: (at, values) => at && values.forEach((each, i) => view().setInt32(at + i * 4, Number(each) | 0, true)),
      u: (at, values) => at && values.forEach((each, i) => view().setUint32(at + i * 4, Number(each) >>> 0, true)),
      f: (at, values) => at && values.forEach((each, i) => view().setFloat32(at + i * 4, Number(each), true)),
      b: (at, values) => at && values.forEach((each, i) => view().setUint8(at + i, each ? 1 : 0)),
      l: (at, values) => at && values.forEach((each, i) => view().setBigInt64(at + i * 8, BigInt(Math.trunc(Number(each) || 0)), true)),
    };
    /** Writes what OpenGL had to say into the program's own room for it, closed with a zero. */
    const glSaid = (log, room, lengthAt, at) => {
      const held = encoder.encode(log || "").subarray(0, Math.max(0, room - 1));
      if (room > 0 && at) {
        bytes().set(held, at);
        bytes()[at + held.length] = 0;
      }
      if (lengthAt) view().setInt32(lengthAt, held.length, true);
    };
    const glFresh = (kind, make) => (count, at) => {
      for (let i = 0; i < count; i++) view().setUint32(at + i * 4, keep(kind, make()), true);
    };
    const glGone = (kind, drop) => (count, at) => {
      for (let i = 0; i < count; i++) {
        const id = view().getUint32(at + i * 4, true);
        if (made[kind][id]) drop(made[kind][id]);
        if (id) made[kind][id] = null;
      }
    };
    /** Whether a number names a thing of that kind that is still there. */
    const glIs = (kind, id, check) => (made[kind][id] && check(made[kind][id]) ? 1 : 0);
    /** The program or shader a number names; a number that names none is the program's error. */
    const glHeld = (kind, id) => {
      const held = made[kind][id];
      if (!held) glFail(0x0501);
      return held;
    };
    /** The number OpenGL names a thing of the page's by (a buffer, a texture), or 0. */
    const glNumber = (object) => {
      for (const kind of Object.keys(made)) {
        const id = made[kind].indexOf(object);
        if (id > 0) return id;
      }
      return 0;
    };
    /** What WebGL answered, as numbers: a thing (a buffer, a texture) as the number the program knows it by. */
    const glValues = (value) => {
      if (value === null || value === undefined) return [0];
      if (typeof value === "object") return "length" in value ? Array.from(value, (each) => (each && typeof each === "object" ? glNumber(each) : Number(each))) : [glNumber(value)];
      return [Number(value)];
    };
    /** What OpenGL says of itself under a name: WebGL's answer, and the few WebGL does not give. */
    const glParameter = (name) => {
      switch (name) {
        case 0x821b: // GL_MAJOR_VERSION
          return 3;
        case 0x821c: // GL_MINOR_VERSION
          return 0;
        case 0x821d: // GL_NUM_EXTENSIONS
          return glExtensions.length;
        case 0x86a2: // GL_NUM_COMPRESSED_TEXTURE_FORMATS
          return (drawing.getParameter(0x86a3) || []).length;
        case 0x87fe: // GL_NUM_PROGRAM_BINARY_FORMATS, GL_NUM_SHADER_BINARY_FORMATS: there are none
        case 0x8df9:
          return 0;
        case 0x87ff: // and which they are
        case 0x8df8:
          return [];
        case 0x8dfa: // GL_SHADER_COMPILER
          return true;
        default:
          return drawing.getParameter(name);
      }
    };
    /** The name OpenGL asks which buffer is bound to a target under. */
    const glBinding = (target) =>
      ({ 0x8892: 0x8894, 0x8893: 0x8895, 0x8f36: 0x8f36, 0x8f37: 0x8f37, 0x88eb: 0x88ed, 0x88ec: 0x88ef, 0x8c8e: 0x8c8f, 0x8a11: 0x8a28 })[target] || 0;
    /** The buffer bound to a target, and what the kit's C mapped of it (glMapBufferRange). */
    const glBound = (target) => {
      const binding = glBinding(target);
      return binding ? drawing.getParameter(binding) : null;
    };
    // How pictures are laid out in the program's memory, as glPixelStorei said: rows made up to
    // a multiple of the alignment, and where a picture begins in a larger one.
    const glStore = { 0x0cf5: 4, 0x0cf2: 0, 0x0cf3: 0, 0x0cf4: 0, 0x806e: 0, 0x806d: 0, 0x0d05: 4, 0x0d02: 0, 0x0d03: 0, 0x0d04: 0 };
    // Whether a buffer is bound for pictures to come from, or to go to: then a picture's place is
    // where in that buffer it is, and not in the program's memory.
    let glUnpacking = false;
    let glPacking = false;
    /** How many bytes a dot takes, by its format and its type. */
    const glDotBytes = (format, type) => {
      const packed = { 0x8363: 2, 0x8033: 2, 0x8034: 2, 0x8368: 4, 0x8c3b: 4, 0x8c3e: 4, 0x84fa: 4, 0x8dad: 8 }[type];
      if (packed) return packed;
      const parts = { 0x1903: 1, 0x8227: 2, 0x1907: 3, 0x1908: 4, 0x1909: 1, 0x190a: 2, 0x1906: 1, 0x1902: 1, 0x8d94: 1, 0x8228: 2, 0x8d98: 3, 0x8d99: 4 }[format] || 4;
      return parts * ({ 0x1400: 1, 0x1401: 1, 0x1402: 2, 0x1403: 2, 0x140b: 2, 0x1404: 4, 0x1405: 4, 0x1406: 4 }[type] || 1);
    };
    /** The kind of array WebGL takes a picture of this type in. */
    const glDotArray = (type) =>
      ({ 0x1400: Int8Array, 0x1402: Int16Array, 0x1403: Uint16Array, 0x140b: Uint16Array, 0x8363: Uint16Array, 0x8033: Uint16Array, 0x8034: Uint16Array, 0x1404: Int32Array, 0x1405: Uint32Array, 0x8368: Uint32Array, 0x8c3b: Uint32Array, 0x8c3e: Uint32Array, 0x84fa: Uint32Array, 0x1406: Float32Array })[type] || Uint8Array;
    /** How many bytes a picture of so many dots takes in the program's memory, laid out as glPixelStorei said. */
    const glImageBytes = (width, height, depth, format, type, unpack) => {
      if (width <= 0 || height <= 0 || depth <= 0) return 0;
      const each = glDotBytes(format, type);
      const [align, rowLength, skipRows, skipPixels, imageHeight, skipImages] = unpack
        ? [glStore[0x0cf5], glStore[0x0cf2], glStore[0x0cf3], glStore[0x0cf4], glStore[0x806e], glStore[0x806d]]
        : [glStore[0x0d05], glStore[0x0d02], glStore[0x0d03], glStore[0x0d04], 0, 0];
      const row = Math.ceil(((rowLength || width) * each) / align) * align;
      const image = row * (imageHeight || height);
      return (skipImages + depth - 1) * image + (skipRows + height - 1) * row + (skipPixels + width) * each;
    };
    /** A picture the program hands over: its dots in its memory, or, while a buffer is bound for pictures to come from, where in that buffer. */
    const glPicture = (at, width, height, depth, format, type) => {
      if (glUnpacking) return at;
      if (!at || type === 0x8dad) return null;
      const Kind = glDotArray(type);
      return glArray(Kind, at, Math.ceil(glImageBytes(width, height, depth, format, type, true) / Kind.BYTES_PER_ELEMENT));
    };
    /** Compressed dots the program hands over, or where they are in the buffer bound for pictures to come from. */
    const glPacked = (at, size) => (glUnpacking ? [size, at] : [new Uint8Array(memory.buffer, at, Math.max(0, size))]);
    /** What WebGL says of an attribute, a uniform or a varying, written where the program asked. */
    const glInfo = (info, room, lengthAt, sizeAt, typeAt, nameAt) => {
      if (!info) return glFail(0x0501);
      return [glSaid(info.name, room, lengthAt, nameAt), glPut.i(sizeAt, [info.size]), glPut.u(typeAt, [info.type])];
    };
    /** The longest of so many names, and the zero after it: what OpenGL says a program's longest name needs. */
    const glLongest = (count, nameOf) => {
      let most = 0;
      for (let i = 0; i < count; i++) most = Math.max(most, (nameOf(i) || "").length + 1);
      return most;
    };
    const graphics = drawing && {
      glViewport: (x, y, width, height) => drawing.viewport(x, y, width, height),
      glScissor: (x, y, width, height) => drawing.scissor(x, y, width, height),
      glClearColor: (red, green, blue, alpha) => drawing.clearColor(red, green, blue, alpha),
      glClearDepthf: (depth) => drawing.clearDepth(depth),
      glClearStencil: (value) => drawing.clearStencil(value),
      glClear: (mask) => drawing.clear(mask),
      glEnable: (what) => drawing.enable(what),
      glDisable: (what) => drawing.disable(what),
      glIsEnabled: (what) => (drawing.isEnabled(what) ? 1 : 0),
      glDepthFunc: (how) => drawing.depthFunc(how),
      glDepthMask: (flag) => drawing.depthMask(!!flag),
      glDepthRangef: (near, far) => drawing.depthRange(near, far),
      glCullFace: (mode) => drawing.cullFace(mode),
      glFrontFace: (mode) => drawing.frontFace(mode),
      glBlendFunc: (from, to) => drawing.blendFunc(from, to),
      glBlendFuncSeparate: (from, to, fromAlpha, toAlpha) => drawing.blendFuncSeparate(from, to, fromAlpha, toAlpha),
      glBlendEquation: (how) => drawing.blendEquation(how),
      glBlendEquationSeparate: (how, howAlpha) => drawing.blendEquationSeparate(how, howAlpha),
      glBlendColor: (red, green, blue, alpha) => drawing.blendColor(red, green, blue, alpha),
      glColorMask: (red, green, blue, alpha) => drawing.colorMask(!!red, !!green, !!blue, !!alpha),
      glPolygonOffset: (factor, units) => drawing.polygonOffset(factor, units),
      glLineWidth: (width) => drawing.lineWidth(width),
      glSampleCoverage: (value, invert) => drawing.sampleCoverage(value, !!invert),
      glStencilFunc: (how, reference, mask) => drawing.stencilFunc(how, reference, mask),
      glStencilFuncSeparate: (face, how, reference, mask) => drawing.stencilFuncSeparate(face, how, reference, mask),
      glStencilOp: (failed, depthFailed, passed) => drawing.stencilOp(failed, depthFailed, passed),
      glStencilOpSeparate: (face, failed, depthFailed, passed) => drawing.stencilOpSeparate(face, failed, depthFailed, passed),
      glStencilMask: (mask) => drawing.stencilMask(mask),
      glStencilMaskSeparate: (face, mask) => drawing.stencilMaskSeparate(face, mask),
      glHint: (target, mode) => drawing.hint(target, mode),
      glPixelStorei(name, value) {
        if (name in glStore) glStore[name] = value;
        drawing.pixelStorei(name, value);
      },
      glReadPixels(x, y, width, height, format, type, at) {
        if (glPacking) drawing.readPixels(x, y, width, height, format, type, at);
        else if (at) {
          const Kind = glDotArray(type);
          drawing.readPixels(x, y, width, height, format, type, glArray(Kind, at, Math.ceil(glImageBytes(width, height, 1, format, type, false) / Kind.BYTES_PER_ELEMENT)));
        }
      },
      glReadBuffer: (source) => drawing.readBuffer(source),
      glFinish: () => drawing.finish(),
      glFlush: () => drawing.flush(),
      glGetError() {
        const own = glOwnError;
        glOwnError = 0;
        return own || drawing.getError();
      },

      glGetBooleanv: (name, at) => glPut.b(at, glValues(glParameter(name))),
      glGetFloatv: (name, at) => glPut.f(at, glValues(glParameter(name))),
      glGetIntegerv: (name, at) => glPut.i(at, glValues(glParameter(name))),
      glGetInteger64v: (name, at) => glPut.l(at, glValues(glParameter(name))),
      glGetIntegeri_v: (target, index, at) => glPut.i(at, glValues(drawing.getIndexedParameter(target, index))),
      glGetInteger64i_v: (target, index, at) => glPut.l(at, glValues(drawing.getIndexedParameter(target, index))),
      glGetInternalformativ(target, format, name, room, at) {
        const samples = drawing.getInternalformatParameter(target, format, 0x80a9);
        const values = name === 0x9380 ? [samples ? samples.length : 0] : glValues(drawing.getInternalformatParameter(target, format, name));
        glPut.i(at, values.slice(0, Math.max(0, room)));
      },
      // The text OpenGL says of itself, written into the room the kit's C keeps for it (glGetString).
      gridcraft_glString(name, index, at, room) {
        let said = null;
        if (index < 0 && name === 0x1f03) said = glExtensions.join(" ");
        else if (index < 0 && name === 0x1f02) said = "OpenGL ES 3.0 (" + drawing.getParameter(name) + ")";
        else if (index < 0 && name === 0x8b8c) said = "OpenGL ES GLSL ES 3.00 (" + drawing.getParameter(name) + ")";
        else if (index < 0 && (name === 0x1f00 || name === 0x1f01)) said = String(drawing.getParameter(name));
        else if (index >= 0 && name === 0x1f03 && index < glExtensions.length) said = glExtensions[index];
        if (said === null) {
          glFail(index >= 0 && name === 0x1f03 ? 0x0501 : 0x0500);
          return -1;
        }
        glSaid(said, room, 0, at);
        return said.length;
      },
      // Mapping a buffer, done by the kit's C in the program's own memory (glMapBufferRange): what
      // is mapped is kept with the buffer here, so that OpenGL can be asked of it.
      gridcraft_glMap(target, offset, length, access, at) {
        const held = glBound(target);
        if (!held || held.gridcraftMapped) return 0x0502;
        held.gridcraftMapped = { offset, length, access, at };
        return 0;
      },
      gridcraft_glMapped(target, into) {
        const held = glBound(target);
        const mapped = held && held.gridcraftMapped;
        if (!mapped) return 0;
        glPut.i(into, [mapped.offset, mapped.length, mapped.access, mapped.at]);
        return 1;
      },
      gridcraft_glUnmap(target) {
        const held = glBound(target);
        if (held) delete held.gridcraftMapped;
      },
      gridcraft_glReadBuffer: (target, offset, size, at) => drawing.getBufferSubData(target, offset, new Uint8Array(memory.buffer, at, Math.max(0, size))),
      gridcraft_glFail: (error) => glFail(error),

      glCreateShader: (type) => keep("shader", drawing.createShader(type)),
      glShaderSource(shader, count, textsAt, lengthsAt) {
        let source = "";
        for (let i = 0; i < count; i++) {
          const at = view().getUint32(textsAt + i * 4, true);
          const length = lengthsAt ? view().getInt32(lengthsAt + i * 4, true) : -1;
          source += length < 0 ? glText(at) : decoder.decode(bytes().subarray(at, at + length));
        }
        const held = glHeld("shader", shader);
        if (held) drawing.shaderSource(held, source);
      },
      glCompileShader: (shader) => made.shader[shader] && drawing.compileShader(made.shader[shader]),
      glGetShaderiv(shader, name, at) {
        const held = glHeld("shader", shader);
        let value = 0;
        if (held && name === 0x8b84) value = (drawing.getShaderInfoLog(held) || "").length + 1;
        else if (held && name === 0x8b88) value = (drawing.getShaderSource(held) || "").length + 1;
        else if (held) value = Number(drawing.getShaderParameter(held, name));
        view().setInt32(at, value, true);
      },
      glGetShaderInfoLog: (shader, room, lengthAt, at) => glSaid(made.shader[shader] ? drawing.getShaderInfoLog(made.shader[shader]) : "", room, lengthAt, at),
      glGetShaderSource: (shader, room, lengthAt, at) => glSaid(made.shader[shader] ? drawing.getShaderSource(made.shader[shader]) : "", room, lengthAt, at),
      glGetShaderPrecisionFormat(kind, precision, rangeAt, precisionAt) {
        const format = drawing.getShaderPrecisionFormat(kind, precision);
        glPut.i(rangeAt, format ? [format.rangeMin, format.rangeMax] : [0, 0]);
        glPut.i(precisionAt, [format ? format.precision : 0]);
      },
      glDeleteShader(shader) {
        if (made.shader[shader]) drawing.deleteShader(made.shader[shader]);
        if (shader) made.shader[shader] = null;
      },
      glIsShader: (shader) => glIs("shader", shader, (held) => drawing.isShader(held)),
      glReleaseShaderCompiler: () => {},
      glShaderBinary: () => glFail(0x0500),
      glCreateProgram: () => keep("program", drawing.createProgram()),
      glAttachShader: (program, shader) => made.program[program] && made.shader[shader] && drawing.attachShader(made.program[program], made.shader[shader]),
      glDetachShader: (program, shader) => made.program[program] && made.shader[shader] && drawing.detachShader(made.program[program], made.shader[shader]),
      glGetAttachedShaders(program, most, countAt, at) {
        const held = glHeld("program", program);
        const shaders = (held && drawing.getAttachedShaders(held)) || [];
        const numbers = shaders.map(glNumber).slice(0, Math.max(0, most));
        glPut.u(at, numbers);
        glPut.i(countAt, [numbers.length]);
      },
      glBindAttribLocation: (program, index, nameAt) => made.program[program] && drawing.bindAttribLocation(made.program[program], index, glText(nameAt)),
      glLinkProgram: (program) => made.program[program] && drawing.linkProgram(made.program[program]),
      glValidateProgram: (program) => made.program[program] && drawing.validateProgram(made.program[program]),
      glGetProgramiv(program, name, at) {
        const held = glHeld("program", program);
        let value = 0;
        if (!held) value = 0;
        else if (name === 0x8b84) value = (drawing.getProgramInfoLog(held) || "").length + 1;
        else if (name === 0x8b87) value = glLongest(drawing.getProgramParameter(held, 0x8b86), (i) => drawing.getActiveUniform(held, i)?.name);
        else if (name === 0x8b8a) value = glLongest(drawing.getProgramParameter(held, 0x8b89), (i) => drawing.getActiveAttrib(held, i)?.name);
        else if (name === 0x8c76) value = glLongest(drawing.getProgramParameter(held, 0x8c83), (i) => drawing.getTransformFeedbackVarying(held, i)?.name);
        else if (name === 0x8a35) value = glLongest(drawing.getProgramParameter(held, 0x8a36), (i) => drawing.getActiveUniformBlockName(held, i));
        else if (name === 0x8741 || name === 0x8257) value = 0;
        else value = Number(drawing.getProgramParameter(held, name));
        view().setInt32(at, value, true);
      },
      glGetProgramInfoLog: (program, room, lengthAt, at) => glSaid(made.program[program] ? drawing.getProgramInfoLog(made.program[program]) : "", room, lengthAt, at),
      glUseProgram: (program) => drawing.useProgram(made.program[program] || null),
      glDeleteProgram(program) {
        if (made.program[program]) drawing.deleteProgram(made.program[program]);
        if (program) made.program[program] = null;
      },
      glIsProgram: (program) => glIs("program", program, (held) => drawing.isProgram(held)),
      glGetProgramBinary(program, room, lengthAt) {
        glPut.i(lengthAt, [0]);
        glFail(0x0502);
      },
      glProgramBinary: () => glFail(0x0500),
      glProgramParameteri: () => {},
      glGetAttribLocation: (program, nameAt) => (made.program[program] ? drawing.getAttribLocation(made.program[program], glText(nameAt)) : -1),
      glGetActiveAttrib(program, index, room, lengthAt, sizeAt, typeAt, nameAt) {
        const held = glHeld("program", program);
        if (held) glInfo(drawing.getActiveAttrib(held, index), room, lengthAt, sizeAt, typeAt, nameAt);
      },
      glGetFragDataLocation: (program, nameAt) => (made.program[program] ? drawing.getFragDataLocation(made.program[program], glText(nameAt)) : -1),
      glTransformFeedbackVaryings(program, count, namesAt, mode) {
        const held = glHeld("program", program);
        if (held) drawing.transformFeedbackVaryings(held, glTexts(namesAt, count), mode);
      },
      glGetTransformFeedbackVarying(program, index, room, lengthAt, sizeAt, typeAt, nameAt) {
        const held = glHeld("program", program);
        if (held) glInfo(drawing.getTransformFeedbackVarying(held, index), room, lengthAt, sizeAt, typeAt, nameAt);
      },

      glGetUniformLocation(program, nameAt) {
        const place = made.program[program] ? drawing.getUniformLocation(made.program[program], glText(nameAt)) : null;
        return place ? keep("place", place) : -1;
      },
      glGetActiveUniform(program, index, room, lengthAt, sizeAt, typeAt, nameAt) {
        const held = glHeld("program", program);
        if (held) glInfo(drawing.getActiveUniform(held, index), room, lengthAt, sizeAt, typeAt, nameAt);
      },
      glGetUniformfv: (program, place, at) => made.program[program] && made.place[place] && glPut.f(at, glValues(drawing.getUniform(made.program[program], made.place[place]))),
      glGetUniformiv: (program, place, at) => made.program[program] && made.place[place] && glPut.i(at, glValues(drawing.getUniform(made.program[program], made.place[place]))),
      glGetUniformuiv: (program, place, at) => made.program[program] && made.place[place] && glPut.u(at, glValues(drawing.getUniform(made.program[program], made.place[place]))),
      glUniform1f: (place, a) => drawing.uniform1f(made.place[place] || null, a),
      glUniform2f: (place, a, b) => drawing.uniform2f(made.place[place] || null, a, b),
      glUniform3f: (place, a, b, c) => drawing.uniform3f(made.place[place] || null, a, b, c),
      glUniform4f: (place, a, b, c, d) => drawing.uniform4f(made.place[place] || null, a, b, c, d),
      glUniform1i: (place, a) => drawing.uniform1i(made.place[place] || null, a),
      glUniform2i: (place, a, b) => drawing.uniform2i(made.place[place] || null, a, b),
      glUniform3i: (place, a, b, c) => drawing.uniform3i(made.place[place] || null, a, b, c),
      glUniform4i: (place, a, b, c, d) => drawing.uniform4i(made.place[place] || null, a, b, c, d),
      glUniform1ui: (place, a) => drawing.uniform1ui(made.place[place] || null, a),
      glUniform2ui: (place, a, b) => drawing.uniform2ui(made.place[place] || null, a, b),
      glUniform3ui: (place, a, b, c) => drawing.uniform3ui(made.place[place] || null, a, b, c),
      glUniform4ui: (place, a, b, c, d) => drawing.uniform4ui(made.place[place] || null, a, b, c, d),
      glUniform1fv: (place, count, at) => drawing.uniform1fv(made.place[place] || null, glFloats(at, count)),
      glUniform2fv: (place, count, at) => drawing.uniform2fv(made.place[place] || null, glFloats(at, count * 2)),
      glUniform3fv: (place, count, at) => drawing.uniform3fv(made.place[place] || null, glFloats(at, count * 3)),
      glUniform4fv: (place, count, at) => drawing.uniform4fv(made.place[place] || null, glFloats(at, count * 4)),
      glUniform1iv: (place, count, at) => drawing.uniform1iv(made.place[place] || null, glArray(Int32Array, at, count)),
      glUniform2iv: (place, count, at) => drawing.uniform2iv(made.place[place] || null, glArray(Int32Array, at, count * 2)),
      glUniform3iv: (place, count, at) => drawing.uniform3iv(made.place[place] || null, glArray(Int32Array, at, count * 3)),
      glUniform4iv: (place, count, at) => drawing.uniform4iv(made.place[place] || null, glArray(Int32Array, at, count * 4)),
      glUniform1uiv: (place, count, at) => drawing.uniform1uiv(made.place[place] || null, glArray(Uint32Array, at, count)),
      glUniform2uiv: (place, count, at) => drawing.uniform2uiv(made.place[place] || null, glArray(Uint32Array, at, count * 2)),
      glUniform3uiv: (place, count, at) => drawing.uniform3uiv(made.place[place] || null, glArray(Uint32Array, at, count * 3)),
      glUniform4uiv: (place, count, at) => drawing.uniform4uiv(made.place[place] || null, glArray(Uint32Array, at, count * 4)),
      glUniformMatrix2fv: (place, count, turned, at) => drawing.uniformMatrix2fv(made.place[place] || null, !!turned, glFloats(at, count * 4)),
      glUniformMatrix3fv: (place, count, turned, at) => drawing.uniformMatrix3fv(made.place[place] || null, !!turned, glFloats(at, count * 9)),
      glUniformMatrix4fv: (place, count, turned, at) => drawing.uniformMatrix4fv(made.place[place] || null, !!turned, glFloats(at, count * 16)),
      glUniformMatrix2x3fv: (place, count, turned, at) => drawing.uniformMatrix2x3fv(made.place[place] || null, !!turned, glFloats(at, count * 6)),
      glUniformMatrix3x2fv: (place, count, turned, at) => drawing.uniformMatrix3x2fv(made.place[place] || null, !!turned, glFloats(at, count * 6)),
      glUniformMatrix2x4fv: (place, count, turned, at) => drawing.uniformMatrix2x4fv(made.place[place] || null, !!turned, glFloats(at, count * 8)),
      glUniformMatrix4x2fv: (place, count, turned, at) => drawing.uniformMatrix4x2fv(made.place[place] || null, !!turned, glFloats(at, count * 8)),
      glUniformMatrix3x4fv: (place, count, turned, at) => drawing.uniformMatrix3x4fv(made.place[place] || null, !!turned, glFloats(at, count * 12)),
      glUniformMatrix4x3fv: (place, count, turned, at) => drawing.uniformMatrix4x3fv(made.place[place] || null, !!turned, glFloats(at, count * 12)),
      glGetUniformBlockIndex: (program, nameAt) => (made.program[program] ? drawing.getUniformBlockIndex(made.program[program], glText(nameAt)) : 0xffffffff),
      glGetActiveUniformBlockiv(program, index, name, at) {
        const held = glHeld("program", program);
        if (held && name === 0x8a41) glPut.i(at, [(drawing.getActiveUniformBlockName(held, index) || "").length + 1]);
        else if (held) glPut.i(at, glValues(drawing.getActiveUniformBlockParameter(held, index, name)));
      },
      glGetActiveUniformBlockName: (program, index, room, lengthAt, at) =>
        glSaid(made.program[program] ? drawing.getActiveUniformBlockName(made.program[program], index) : "", room, lengthAt, at),
      glUniformBlockBinding: (program, index, binding) => made.program[program] && drawing.uniformBlockBinding(made.program[program], index, binding),
      glGetUniformIndices(program, count, namesAt, at) {
        const held = glHeld("program", program);
        const indices = held ? drawing.getUniformIndices(held, glTexts(namesAt, count)) : null;
        glPut.u(at, Array.from(indices || [], (each) => (each === null ? 0xffffffff : each)));
      },
      glGetActiveUniformsiv(program, count, indicesAt, name, at) {
        const held = glHeld("program", program);
        if (held) glPut.i(at, glValues(drawing.getActiveUniforms(held, Array.from(glArray(Uint32Array, indicesAt, count)), name)));
      },

      glGenBuffers: glFresh("buffer", () => drawing.createBuffer()),
      glDeleteBuffers: glGone("buffer", (held) => drawing.deleteBuffer(held)),
      glIsBuffer: (buffer) => glIs("buffer", buffer, (held) => drawing.isBuffer(held)),
      glBindBuffer(target, buffer) {
        drawing.bindBuffer(target, made.buffer[buffer] || null);
        if (target === 0x88ec) glUnpacking = !!made.buffer[buffer];
        if (target === 0x88eb) glPacking = !!made.buffer[buffer];
      },
      glBindBufferBase: (target, index, buffer) => drawing.bindBufferBase(target, index, made.buffer[buffer] || null),
      glBindBufferRange: (target, index, buffer, offset, size) => drawing.bindBufferRange(target, index, made.buffer[buffer] || null, offset, size),
      glBufferData(target, size, at, usage) {
        if (at) drawing.bufferData(target, new Uint8Array(memory.buffer, at, size), usage);
        else drawing.bufferData(target, size, usage);
      },
      glBufferSubData: (target, offset, size, at) => drawing.bufferSubData(target, offset, new Uint8Array(memory.buffer, at, size)),
      glCopyBufferSubData: (from, to, fromOffset, toOffset, size) => drawing.copyBufferSubData(from, to, fromOffset, toOffset, size),
      glGetBufferParameteriv(target, name, at) {
        const held = glBound(target);
        const mapped = held && held.gridcraftMapped;
        const own = { 0x88bc: mapped ? 1 : 0, 0x911f: mapped ? mapped.access : 0, 0x9120: mapped ? mapped.length : 0, 0x9121: mapped ? mapped.offset : 0 }[name];
        glPut.i(at, [own === undefined ? Number(drawing.getBufferParameter(target, name)) : own]);
      },
      glGetBufferParameteri64v(target, name, at) {
        const held = glBound(target);
        const mapped = held && held.gridcraftMapped;
        const own = { 0x88bc: mapped ? 1 : 0, 0x911f: mapped ? mapped.access : 0, 0x9120: mapped ? mapped.length : 0, 0x9121: mapped ? mapped.offset : 0 }[name];
        glPut.l(at, [own === undefined ? Number(drawing.getBufferParameter(target, name)) : own]);
      },
      glGetBufferPointerv(target, name, at) {
        const held = glBound(target);
        if (name !== 0x88bd) glFail(0x0500);
        else glPut.u(at, [held && held.gridcraftMapped ? held.gridcraftMapped.at : 0]);
      },

      glGenVertexArrays: glFresh("array", () => drawing.createVertexArray()),
      glDeleteVertexArrays: glGone("array", (held) => drawing.deleteVertexArray(held)),
      glBindVertexArray: (array) => drawing.bindVertexArray(made.array[array] || null),
      glIsVertexArray: (array) => glIs("array", array, (held) => drawing.isVertexArray(held)),
      glEnableVertexAttribArray: (index) => drawing.enableVertexAttribArray(index),
      glDisableVertexAttribArray: (index) => drawing.disableVertexAttribArray(index),
      glVertexAttribPointer: (index, size, type, normal, stride, offset) => drawing.vertexAttribPointer(index, size, type, !!normal, stride, offset),
      glVertexAttribIPointer: (index, size, type, stride, offset) => drawing.vertexAttribIPointer(index, size, type, stride, offset),
      glVertexAttribDivisor: (index, divisor) => drawing.vertexAttribDivisor(index, divisor),
      glVertexAttrib1f: (index, x) => drawing.vertexAttrib1f(index, x),
      glVertexAttrib2f: (index, x, y) => drawing.vertexAttrib2f(index, x, y),
      glVertexAttrib3f: (index, x, y, z) => drawing.vertexAttrib3f(index, x, y, z),
      glVertexAttrib4f: (index, x, y, z, w) => drawing.vertexAttrib4f(index, x, y, z, w),
      glVertexAttrib1fv: (index, at) => drawing.vertexAttrib1fv(index, glFloats(at, 1)),
      glVertexAttrib2fv: (index, at) => drawing.vertexAttrib2fv(index, glFloats(at, 2)),
      glVertexAttrib3fv: (index, at) => drawing.vertexAttrib3fv(index, glFloats(at, 3)),
      glVertexAttrib4fv: (index, at) => drawing.vertexAttrib4fv(index, glFloats(at, 4)),
      glVertexAttribI4i: (index, x, y, z, w) => drawing.vertexAttribI4i(index, x, y, z, w),
      glVertexAttribI4ui: (index, x, y, z, w) => drawing.vertexAttribI4ui(index, x, y, z, w),
      glVertexAttribI4iv: (index, at) => drawing.vertexAttribI4iv(index, glArray(Int32Array, at, 4)),
      glVertexAttribI4uiv: (index, at) => drawing.vertexAttribI4uiv(index, glArray(Uint32Array, at, 4)),
      glGetVertexAttribfv: (index, name, at) => glPut.f(at, glValues(drawing.getVertexAttrib(index, name))),
      glGetVertexAttribiv: (index, name, at) => glPut.i(at, glValues(drawing.getVertexAttrib(index, name))),
      glGetVertexAttribIiv: (index, name, at) => glPut.i(at, glValues(drawing.getVertexAttrib(index, name))),
      glGetVertexAttribIuiv: (index, name, at) => glPut.u(at, glValues(drawing.getVertexAttrib(index, name))),
      glGetVertexAttribPointerv: (index, name, at) => glPut.u(at, [drawing.getVertexAttribOffset(index, name)]),
      glDrawArrays: (mode, first, count) => drawing.drawArrays(mode, first, count),
      glDrawElements: (mode, count, type, offset) => drawing.drawElements(mode, count, type, offset),
      glDrawRangeElements: (mode, start, end, count, type, offset) => drawing.drawRangeElements(mode, start, end, count, type, offset),
      glDrawArraysInstanced: (mode, first, count, instances) => drawing.drawArraysInstanced(mode, first, count, instances),
      glDrawElementsInstanced: (mode, count, type, offset, instances) => drawing.drawElementsInstanced(mode, count, type, offset, instances),

      glGenTextures: glFresh("texture", () => drawing.createTexture()),
      glDeleteTextures: glGone("texture", (held) => drawing.deleteTexture(held)),
      glBindTexture: (target, texture) => drawing.bindTexture(target, made.texture[texture] || null),
      glIsTexture: (texture) => glIs("texture", texture, (held) => drawing.isTexture(held)),
      glActiveTexture: (unit) => drawing.activeTexture(unit),
      glTexImage2D: (target, level, kind, width, height, border, format, type, at) =>
        drawing.texImage2D(target, level, kind, width, height, border, format, type, glPicture(at, width, height, 1, format, type)),
      glTexImage3D: (target, level, kind, width, height, depth, border, format, type, at) =>
        drawing.texImage3D(target, level, kind, width, height, depth, border, format, type, glPicture(at, width, height, depth, format, type)),
      glTexSubImage2D(target, level, x, y, width, height, format, type, at) {
        const dots = glPicture(at, width, height, 1, format, type);
        if (dots !== null) drawing.texSubImage2D(target, level, x, y, width, height, format, type, dots);
      },
      glTexSubImage3D(target, level, x, y, z, width, height, depth, format, type, at) {
        const dots = glPicture(at, width, height, depth, format, type);
        if (dots !== null) drawing.texSubImage3D(target, level, x, y, z, width, height, depth, format, type, dots);
      },
      glTexStorage2D: (target, levels, kind, width, height) => drawing.texStorage2D(target, levels, kind, width, height),
      glTexStorage3D: (target, levels, kind, width, height, depth) => drawing.texStorage3D(target, levels, kind, width, height, depth),
      glCopyTexImage2D: (target, level, kind, x, y, width, height, border) => drawing.copyTexImage2D(target, level, kind, x, y, width, height, border),
      glCopyTexSubImage2D: (target, level, toX, toY, x, y, width, height) => drawing.copyTexSubImage2D(target, level, toX, toY, x, y, width, height),
      glCopyTexSubImage3D: (target, level, toX, toY, toZ, x, y, width, height) => drawing.copyTexSubImage3D(target, level, toX, toY, toZ, x, y, width, height),
      glCompressedTexImage2D: (target, level, kind, width, height, border, size, at) =>
        drawing.compressedTexImage2D(target, level, kind, width, height, border, ...glPacked(at, size)),
      glCompressedTexImage3D: (target, level, kind, width, height, depth, border, size, at) =>
        drawing.compressedTexImage3D(target, level, kind, width, height, depth, border, ...glPacked(at, size)),
      glCompressedTexSubImage2D: (target, level, x, y, width, height, format, size, at) =>
        drawing.compressedTexSubImage2D(target, level, x, y, width, height, format, ...glPacked(at, size)),
      glCompressedTexSubImage3D: (target, level, x, y, z, width, height, depth, format, size, at) =>
        drawing.compressedTexSubImage3D(target, level, x, y, z, width, height, depth, format, ...glPacked(at, size)),
      glTexParameterf: (target, name, value) => drawing.texParameterf(target, name, value),
      glTexParameterfv: (target, name, at) => drawing.texParameterf(target, name, glFloats(at, 1)[0] || 0),
      glTexParameteri: (target, name, value) => drawing.texParameteri(target, name, value),
      glTexParameteriv: (target, name, at) => drawing.texParameteri(target, name, glArray(Int32Array, at, 1)[0] || 0),
      glGetTexParameterfv: (target, name, at) => glPut.f(at, glValues(drawing.getTexParameter(target, name))),
      glGetTexParameteriv: (target, name, at) => glPut.i(at, glValues(drawing.getTexParameter(target, name))),
      glGenerateMipmap: (target) => drawing.generateMipmap(target),

      glGenSamplers: glFresh("sampler", () => drawing.createSampler()),
      glDeleteSamplers: glGone("sampler", (held) => drawing.deleteSampler(held)),
      glIsSampler: (sampler) => glIs("sampler", sampler, (held) => drawing.isSampler(held)),
      glBindSampler: (unit, sampler) => drawing.bindSampler(unit, made.sampler[sampler] || null),
      glSamplerParameteri: (sampler, name, value) => made.sampler[sampler] && drawing.samplerParameteri(made.sampler[sampler], name, value),
      glSamplerParameteriv: (sampler, name, at) => made.sampler[sampler] && drawing.samplerParameteri(made.sampler[sampler], name, glArray(Int32Array, at, 1)[0] || 0),
      glSamplerParameterf: (sampler, name, value) => made.sampler[sampler] && drawing.samplerParameterf(made.sampler[sampler], name, value),
      glSamplerParameterfv: (sampler, name, at) => made.sampler[sampler] && drawing.samplerParameterf(made.sampler[sampler], name, glFloats(at, 1)[0] || 0),
      glGetSamplerParameteriv: (sampler, name, at) => made.sampler[sampler] && glPut.i(at, glValues(drawing.getSamplerParameter(made.sampler[sampler], name))),
      glGetSamplerParameterfv: (sampler, name, at) => made.sampler[sampler] && glPut.f(at, glValues(drawing.getSamplerParameter(made.sampler[sampler], name))),

      glGenFramebuffers: glFresh("framebuffer", () => drawing.createFramebuffer()),
      glDeleteFramebuffers: glGone("framebuffer", (held) => drawing.deleteFramebuffer(held)),
      glIsFramebuffer: (framebuffer) => glIs("framebuffer", framebuffer, (held) => drawing.isFramebuffer(held)),
      glBindFramebuffer: (target, framebuffer) => drawing.bindFramebuffer(target, made.framebuffer[framebuffer] || null),
      glCheckFramebufferStatus: (target) => drawing.checkFramebufferStatus(target),
      glFramebufferTexture2D: (target, attachment, kind, texture, level) => drawing.framebufferTexture2D(target, attachment, kind, made.texture[texture] || null, level),
      glFramebufferTextureLayer: (target, attachment, texture, level, layer) => drawing.framebufferTextureLayer(target, attachment, made.texture[texture] || null, level, layer),
      glFramebufferRenderbuffer: (target, attachment, kind, renderbuffer) => drawing.framebufferRenderbuffer(target, attachment, kind, made.renderbuffer[renderbuffer] || null),
      glGetFramebufferAttachmentParameteriv: (target, attachment, name, at) => glPut.i(at, glValues(drawing.getFramebufferAttachmentParameter(target, attachment, name))),
      glDrawBuffers: (count, at) => drawing.drawBuffers(Array.from(glArray(Uint32Array, at, count))),
      glBlitFramebuffer: (x0, y0, x1, y1, toX0, toY0, toX1, toY1, mask, filter) => drawing.blitFramebuffer(x0, y0, x1, y1, toX0, toY0, toX1, toY1, mask, filter),
      glInvalidateFramebuffer: (target, count, at) => drawing.invalidateFramebuffer(target, Array.from(glArray(Uint32Array, at, count))),
      glInvalidateSubFramebuffer: (target, count, at, x, y, width, height) =>
        drawing.invalidateSubFramebuffer(target, Array.from(glArray(Uint32Array, at, count)), x, y, width, height),
      glClearBufferiv: (buffer, index, at) => drawing.clearBufferiv(buffer, index, glArray(Int32Array, at, buffer === 0x1800 ? 4 : 1)),
      glClearBufferuiv: (buffer, index, at) => drawing.clearBufferuiv(buffer, index, glArray(Uint32Array, at, buffer === 0x1800 ? 4 : 1)),
      glClearBufferfv: (buffer, index, at) => drawing.clearBufferfv(buffer, index, glFloats(at, buffer === 0x1800 ? 4 : 1)),
      glClearBufferfi: (buffer, index, depth, stencil) => drawing.clearBufferfi(buffer, index, depth, stencil),
      glGenRenderbuffers: glFresh("renderbuffer", () => drawing.createRenderbuffer()),
      glDeleteRenderbuffers: glGone("renderbuffer", (held) => drawing.deleteRenderbuffer(held)),
      glIsRenderbuffer: (renderbuffer) => glIs("renderbuffer", renderbuffer, (held) => drawing.isRenderbuffer(held)),
      glBindRenderbuffer: (target, renderbuffer) => drawing.bindRenderbuffer(target, made.renderbuffer[renderbuffer] || null),
      glRenderbufferStorage: (target, kind, width, height) => drawing.renderbufferStorage(target, kind, width, height),
      glRenderbufferStorageMultisample: (target, samples, kind, width, height) => drawing.renderbufferStorageMultisample(target, samples, kind, width, height),
      glGetRenderbufferParameteriv: (target, name, at) => glPut.i(at, glValues(drawing.getRenderbufferParameter(target, name))),

      glGenQueries: glFresh("query", () => drawing.createQuery()),
      glDeleteQueries: glGone("query", (held) => drawing.deleteQuery(held)),
      glIsQuery: (query) => glIs("query", query, (held) => drawing.isQuery(held)),
      glBeginQuery(target, query) {
        if (made.query[query]) drawing.beginQuery(target, made.query[query]);
        else glFail(0x0502);
      },
      glEndQuery: (target) => drawing.endQuery(target),
      glGetQueryiv: (target, name, at) => glPut.i(at, glValues(drawing.getQuery(target, name))),
      glGetQueryObjectuiv(query, name, at) {
        if (made.query[query]) glPut.u(at, glValues(drawing.getQueryParameter(made.query[query], name)));
        else glFail(0x0502);
      },

      glGenTransformFeedbacks: glFresh("feedback", () => drawing.createTransformFeedback()),
      glDeleteTransformFeedbacks: glGone("feedback", (held) => drawing.deleteTransformFeedback(held)),
      glIsTransformFeedback: (feedback) => glIs("feedback", feedback, (held) => drawing.isTransformFeedback(held)),
      glBindTransformFeedback: (target, feedback) => drawing.bindTransformFeedback(target, made.feedback[feedback] || null),
      glBeginTransformFeedback: (mode) => drawing.beginTransformFeedback(mode),
      glEndTransformFeedback: () => drawing.endTransformFeedback(),
      glPauseTransformFeedback: () => drawing.pauseTransformFeedback(),
      glResumeTransformFeedback: () => drawing.resumeTransformFeedback(),

      glFenceSync: (condition, flags) => keep("sync", drawing.fenceSync(condition, flags)),
      glIsSync: (sync) => glIs("sync", sync, (held) => drawing.isSync(held)),
      glDeleteSync(sync) {
        if (made.sync[sync]) drawing.deleteSync(made.sync[sync]);
        if (sync) made.sync[sync] = null;
      },
      glClientWaitSync(sync, flags, timeout) {
        // A page may not wait long: WebGL waits no longer than it allows (often not at all).
        if (!made.sync[sync]) {
          glFail(0x0501);
          return 0x911d;
        }
        const most = Number(drawing.getParameter(0x9247)) || 0;
        return drawing.clientWaitSync(made.sync[sync], flags, Math.min(Number(timeout), most));
      },
      glWaitSync(sync) {
        if (made.sync[sync]) drawing.waitSync(made.sync[sync], 0, -1);
        else glFail(0x0501);
      },
      glGetSynciv(sync, name, room, lengthAt, at) {
        if (!made.sync[sync]) {
          glFail(0x0501);
          return;
        }
        if (room > 0) glPut.i(at, [Number(drawing.getSyncParameter(made.sync[sync], name))]);
        glPut.i(lengthAt, [room > 0 ? 1 : 0]);
      },
    };

    // The network, for a program whose project names where it would reach, once the person has
    // turned it on (<gridcraft/net.h>). GridCraft carries its packets: they go to `network`,
    // GridCraft's own address beside the program, as the page's other calls go to GridCraft,
    // and GridCraft decides where each may go. The page reaches nothing more for it.
    const NET_PACKET = 4096, NET_WAITING = 256;
    const outbox = [];
    const inbox = [];
    let netOn = 0; // 1 on, -1 off, 0 not known yet
    let pumping = false;
    // Whether the program asked to listen (it hosts), and what GridCraft said: 1 it listens, -1 it can't, 0 not yet.
    let listenAsked = false;
    let listening = 0;
    /** Packets as GridCraft takes them: each its address's length (two bytes), the address, its data's length, the data. */
    const netBytes = (packets) => {
      const parts = [];
      let size = 0;
      for (const [to, data] of packets) {
        const address = encoder.encode(to);
        const head = new Uint8Array(4 + address.length);
        head[0] = address.length >> 8;
        head[1] = address.length & 255;
        head.set(address, 2);
        head[2 + address.length] = data.length >> 8;
        head[3 + address.length] = data.length & 255;
        parts.push(head, data);
        size += head.length + data.length;
      }
      const out = new Uint8Array(size);
      let at = 0;
      for (const part of parts) {
        out.set(part, at);
        at += part.length;
      }
      return out;
    };
    const netPackets = (got) => {
      const found = [];
      let at = 0;
      while (at + 2 <= got.length) {
        const long = (got[at] << 8) | got[at + 1];
        const from = decoder.decode(got.subarray(at + 2, at + 2 + long));
        at += 2 + long;
        if (at + 2 > got.length) break;
        const size = (got[at] << 8) | got[at + 1];
        found.push([from, got.slice(at + 2, at + 2 + size)]);
        at += 2 + size;
      }
      return found;
    };
    /**
     * Hands GridCraft what is waiting to go and takes what has come. While it is on: again as soon
     * as there is something to send, but not sooner than a frame after the last time, and with
     * nothing to send, every two frames. While it is off: every two seconds, to see whether it
     * was turned on.
     */
    async function pump() {
      pumping = true;
      while (!ended) {
        const began = Date.now();
        const going = outbox.splice(0, NET_WAITING);
        try {
          const headers = { "Content-Type": "application/octet-stream" };
          if (listenAsked) headers["X-GridCraft-Listen"] = "yes";
          const answer = await fetch(network, { method: "POST", headers, body: netBytes(going), cache: "no-store" });
          netOn = answer.ok ? 1 : -1;
          const said = answer.headers.get("X-GridCraft-Listening");
          if (listenAsked) listening = !answer.ok || said === "no" ? -1 : said ? 1 : listening;
          const came = answer.ok ? netPackets(new Uint8Array(await answer.arrayBuffer())) : [];
          for (const packet of came) if (inbox.length < NET_WAITING) inbox.push(packet);
        } catch (error) {
          netOn = -1;
        }
        const pause = netOn < 0 ? 2000 : outbox.length ? 15 : 30;
        await new Promise((done) => setTimeout(done, Math.max(0, began + pause - Date.now())));
      }
      pumping = false;
    }
    const netStart = () => {
      if (network && !pumping && !ended) pump();
    };
    const net = {
      net_status() {
        netStart();
        return network ? netOn : -1;
      },
      net_send(addressAt, dataAt, length) {
        netStart();
        if (!network || netOn < 0) return -1;
        if (length < 0 || length > NET_PACKET || outbox.length >= NET_WAITING) return -2;
        outbox.push([glText(addressAt), bytes().slice(dataAt, dataAt + length)]);
        return 0;
      },
      net_listen() {
        netStart();
        if (!network) return -1;
        listenAsked = true;
        return listening;
      },
      net_receive(fromAt, fromRoom, dataAt, room) {
        netStart();
        const next = inbox.shift();
        if (!next) return -1;
        const [from, data] = next;
        if (data.length > room) return -2;
        bytes().set(data, dataAt);
        glSaid(from, fromRoom, 0, fromAt);
        return data.length;
      },
    };

    const module = new WebAssembly.Module(wasm);
    // Anything else the program was built to ask for is answered "there is no such thing".
    const given = {};
    for (const asked of WebAssembly.Module.imports(module)) {
      if (asked.kind !== "function") throw new Error(`The program asks for ${asked.module}.${asked.name}, which is not something a page can hand it.`);
      given[asked.module] ??= {};
      given[asked.module][asked.name] =
        (asked.module === "wasi_snapshot_preview1" && system[asked.name]) ||
        // A drawing call, when there is a canvas and the call is one that is known; else nothing is drawn.
        (asked.module === "gl" && ((graphics && graphics[asked.name]) || (() => 0))) ||
        // The network's calls, carried by GridCraft.
        (asked.module === "net" && net[asked.name]) ||
        (() => NOSYS);
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
      /** Whether the program has a canvas to draw on with OpenGL ES. */
      draws: !!drawing,
      /** Whether GridCraft carries the program's packets: 1 it does, -1 it does not, 0 not known yet. */
      network: () => (network ? netOn : -1),
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
    // Packets go to GridCraft's own address beside the program.
    return start(new Uint8Array(await answer.arrayBuffer()), { network: new URL("net", answer.url).href, ...options });
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
        // Nothing in it reaches further: it loads the one program, hands packets to GridCraft's own
        // address beside it, and never names the spreadsheet, another address, or a way to run text
        // as script.
        assert_eq!(LOADER.matches("fetch(").count(), 2, "the program, and GridCraft's address for packets");
        assert!(LOADER.contains("network: new URL(\"net\", answer.url).href") && LOADER.contains("await fetch(network, {"));
        // The network's three calls, and nothing of it without GridCraft's address.
        for call in ["net_status", "net_send", "net_receive", "net_listen"] {
            assert!(LOADER.contains(&format!("      {call}(")), "{call}");
        }
        assert!(LOADER.contains("if (!network || netOn < 0) return -1;"));
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
        // Drawing: on the one canvas the page names, with its WebGL 2, and with no canvas nothing.
        assert_eq!(LOADER.matches("getContext(").count(), 1, "one canvas, and its own way of drawing");
        assert!(LOADER.contains("canvas.getContext(\"webgl2\"") && LOADER.contains("asked.module === \"gl\""));
        // Every call the kit's header declares is one the loader answers, and it answers no other.
        let declared: Vec<&str> = crate::c::GL_HEADER.lines().filter_map(|line| line.strip_prefix("GRIDCRAFT_GL(")?.split(')').next()).collect();
        assert!(declared.len() > 50 && declared.contains(&"glDrawArrays"), "{}", declared.len());
        for call in &declared {
            assert!(LOADER.contains(&format!("      {call}")), "{call} is declared and not answered");
        }
        let answered = LOADER.lines().filter(|line| line.starts_with("      gl") && line[8..].chars().next().is_some_and(char::is_uppercase)).count();
        assert_eq!(answered, declared.len(), "the loader answers what the header declares");
        // What is printed is a terminal to the program (no seeking, no telling where), so that its C
        // library sends each line as it is printed: rights without FD_SEEK (4) and FD_TELL (32).
        assert!(LOADER.contains("const rights = fd < 3 ? 0xffffffffffffffdbn : 0xffffffffffffffffn;"));
        assert_eq!(!0xffff_ffff_ffff_ffdb_u64, 4 | 32);
        // What the kit's C for OpenGL ES 3.0 asks of it, under names of the kit's own, is answered too.
        for name in crate::gles3::asked_by_the_kit() {
            assert!(LOADER.contains(&format!("      {name}")), "{name} is asked by the kit's C and not answered");
        }
        // It is plain text a browser reads as it is: nothing a build step would have to make of it.
        assert!(LOADER.is_ascii() && !LOADER.contains('\r') && !LOADER.contains("import ") && !LOADER.contains("require("));
    }
}
