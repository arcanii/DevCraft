//! The loader a C project's page runs its program with: `program.js`, served beside the page
//! as `spreadsheet.js` is ([`crate::bridge`]). It is not a file of the project.
//!
//! A C project is built as one WebAssembly program ([`crate::c`]). A WebAssembly program can do
//! nothing by itself: it asks whoever runs it for everything. The loader hands it the little of
//! a system a C program asks for (a clock, a place to print, random numbers, its arguments, and
//! files kept in memory), and, when the page names a canvas, a part of OpenGL ES 3 to draw on
//! that canvas with (the owner, 2026-10-14; the header for it is [`crate::c::GL_HEADER`]). And
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

    // OpenGL ES 3, for a program whose page names a canvas to draw on: each call the program
    // makes is answered with that canvas's WebGL 2, which is OpenGL ES 3 as a page has it.
    // What OpenGL names by number (a buffer, a shader, a texture) is kept here under that
    // number; 0 is none. It draws on the one canvas and reaches nothing else of the page.
    const drawing = canvas ? canvas.getContext("webgl2", { alpha: false, antialias: true }) : null;
    const made = { buffer: [null], shader: [null], program: [null], texture: [null], array: [null], place: [null] };
    const keep = (kind, object) => (object ? made[kind].push(object) - 1 : 0);
    const glText = (at) => {
      const all = bytes();
      let end = at;
      while (end < all.length && all[end] !== 0) end++;
      return decoder.decode(all.subarray(at, end));
    };
    const glFloats = (at, count) => new Float32Array(memory.buffer, at, count);
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
    /** Where each row of a picture begins, as the program said with glPixelStorei: a multiple of so many bytes. */
    const glAlign = { unpack: 4, pack: 4 };
    /** How many bytes a picture of so many dots takes: so many a dot, each row but the last made up to the alignment. */
    const glBytes = (width, height, format, align) => {
      const each = { 0x1908: 4, 0x1907: 3, 0x190a: 2, 0x1909: 1, 0x1906: 1 }[format] || 4;
      const row = width * each;
      return width > 0 && height > 0 ? Math.ceil(row / align) * align * (height - 1) + row : 0;
    };
    /** The dots of a picture handed over as bytes. */
    const glDots = (at, width, height, format, type) =>
      at && type === 0x1401 ? new Uint8Array(memory.buffer, at, glBytes(width, height, format, glAlign.unpack)) : null;
    /** The number OpenGL names a thing of the page's by (a buffer, a texture), or 0. */
    const glNumber = (object) => {
      for (const kind of Object.keys(made)) {
        const id = made[kind].indexOf(object);
        if (id > 0) return id;
      }
      return 0;
    };
    const graphics = drawing && {
      glViewport: (x, y, width, height) => drawing.viewport(x, y, width, height),
      glScissor: (x, y, width, height) => drawing.scissor(x, y, width, height),
      glClearColor: (red, green, blue, alpha) => drawing.clearColor(red, green, blue, alpha),
      glClearDepthf: (depth) => drawing.clearDepth(depth),
      glClear: (mask) => drawing.clear(mask),
      glEnable: (what) => drawing.enable(what),
      glDisable: (what) => drawing.disable(what),
      glDepthFunc: (how) => drawing.depthFunc(how),
      glDepthMask: (flag) => drawing.depthMask(!!flag),
      glCullFace: (mode) => drawing.cullFace(mode),
      glFrontFace: (mode) => drawing.frontFace(mode),
      glBlendFunc: (from, to) => drawing.blendFunc(from, to),
      glBlendFuncSeparate: (from, to, fromAlpha, toAlpha) => drawing.blendFuncSeparate(from, to, fromAlpha, toAlpha),
      glBlendEquation: (how) => drawing.blendEquation(how),
      glDepthRangef: (near, far) => drawing.depthRange(near, far),
      glColorMask: (red, green, blue, alpha) => drawing.colorMask(!!red, !!green, !!blue, !!alpha),
      glPolygonOffset: (factor, units) => drawing.polygonOffset(factor, units),
      glLineWidth: (width) => drawing.lineWidth(width),
      glStencilFunc: (how, reference, mask) => drawing.stencilFunc(how, reference, mask),
      glStencilOp: (failed, depthFailed, passed) => drawing.stencilOp(failed, depthFailed, passed),
      glStencilMask: (mask) => drawing.stencilMask(mask),
      glClearStencil: (value) => drawing.clearStencil(value),
      glPixelStorei(name, value) {
        if (name === 0x0cf5) glAlign.unpack = value;
        if (name === 0x0d05) glAlign.pack = value;
        drawing.pixelStorei(name, value);
      },
      glReadPixels(x, y, width, height, format, type, at) {
        if (at && type === 0x1401) drawing.readPixels(x, y, width, height, format, type, new Uint8Array(memory.buffer, at, glBytes(width, height, format, glAlign.pack)));
      },
      glGetIntegerv(name, at) {
        const value = drawing.getParameter(name);
        const values = value && typeof value === "object" ? ("length" in value ? Array.from(value, Number) : [glNumber(value)]) : [Number(value) || 0];
        values.forEach((each, i) => view().setInt32(at + i * 4, each | 0, true));
      },
      glFinish: () => drawing.finish(),
      glFlush: () => drawing.flush(),
      glGetError: () => drawing.getError(),

      glCreateShader: (type) => keep("shader", drawing.createShader(type)),
      glShaderSource(shader, count, textsAt, lengthsAt) {
        let source = "";
        for (let i = 0; i < count; i++) {
          const at = view().getUint32(textsAt + i * 4, true);
          const length = lengthsAt ? view().getInt32(lengthsAt + i * 4, true) : -1;
          source += length < 0 ? glText(at) : decoder.decode(bytes().subarray(at, at + length));
        }
        if (made.shader[shader]) drawing.shaderSource(made.shader[shader], source);
      },
      glCompileShader: (shader) => made.shader[shader] && drawing.compileShader(made.shader[shader]),
      glGetShaderiv(shader, name, at) {
        const held = made.shader[shader];
        let value = 0;
        if (held && name === 0x8b84) value = (drawing.getShaderInfoLog(held) || "").length + 1;
        else if (held) value = Number(drawing.getShaderParameter(held, name));
        view().setInt32(at, value, true);
      },
      glGetShaderInfoLog: (shader, room, lengthAt, at) => glSaid(made.shader[shader] ? drawing.getShaderInfoLog(made.shader[shader]) : "", room, lengthAt, at),
      glDeleteShader(shader) {
        if (made.shader[shader]) drawing.deleteShader(made.shader[shader]);
        if (shader) made.shader[shader] = null;
      },
      glCreateProgram: () => keep("program", drawing.createProgram()),
      glAttachShader: (program, shader) => made.program[program] && made.shader[shader] && drawing.attachShader(made.program[program], made.shader[shader]),
      glBindAttribLocation: (program, index, nameAt) => made.program[program] && drawing.bindAttribLocation(made.program[program], index, glText(nameAt)),
      glLinkProgram: (program) => made.program[program] && drawing.linkProgram(made.program[program]),
      glGetProgramiv(program, name, at) {
        const held = made.program[program];
        let value = 0;
        if (held && name === 0x8b84) value = (drawing.getProgramInfoLog(held) || "").length + 1;
        else if (held) value = Number(drawing.getProgramParameter(held, name));
        view().setInt32(at, value, true);
      },
      glGetProgramInfoLog: (program, room, lengthAt, at) => glSaid(made.program[program] ? drawing.getProgramInfoLog(made.program[program]) : "", room, lengthAt, at),
      glUseProgram: (program) => drawing.useProgram(made.program[program] || null),
      glDeleteProgram(program) {
        if (made.program[program]) drawing.deleteProgram(made.program[program]);
        if (program) made.program[program] = null;
      },
      glGetAttribLocation: (program, nameAt) => (made.program[program] ? drawing.getAttribLocation(made.program[program], glText(nameAt)) : -1),
      glGetUniformLocation(program, nameAt) {
        const place = made.program[program] ? drawing.getUniformLocation(made.program[program], glText(nameAt)) : null;
        return place ? keep("place", place) : -1;
      },

      glUniform1i: (place, a) => drawing.uniform1i(made.place[place] || null, a),
      glUniform1f: (place, a) => drawing.uniform1f(made.place[place] || null, a),
      glUniform2f: (place, a, b) => drawing.uniform2f(made.place[place] || null, a, b),
      glUniform3f: (place, a, b, c) => drawing.uniform3f(made.place[place] || null, a, b, c),
      glUniform4f: (place, a, b, c, d) => drawing.uniform4f(made.place[place] || null, a, b, c, d),
      glUniform3fv: (place, count, at) => drawing.uniform3fv(made.place[place] || null, glFloats(at, count * 3)),
      glUniform4fv: (place, count, at) => drawing.uniform4fv(made.place[place] || null, glFloats(at, count * 4)),
      glUniform1fv: (place, count, at) => drawing.uniform1fv(made.place[place] || null, glFloats(at, count)),
      glUniform2fv: (place, count, at) => drawing.uniform2fv(made.place[place] || null, glFloats(at, count * 2)),
      glUniformMatrix3fv: (place, count, turned, at) => drawing.uniformMatrix3fv(made.place[place] || null, !!turned, glFloats(at, count * 9)),
      glUniformMatrix4fv: (place, count, turned, at) => drawing.uniformMatrix4fv(made.place[place] || null, !!turned, glFloats(at, count * 16)),

      glGenBuffers: glFresh("buffer", () => drawing.createBuffer()),
      glBindBuffer: (target, buffer) => drawing.bindBuffer(target, made.buffer[buffer] || null),
      glBufferData(target, size, at, usage) {
        if (at) drawing.bufferData(target, new Uint8Array(memory.buffer, at, size), usage);
        else drawing.bufferData(target, size, usage);
      },
      glBufferSubData: (target, offset, size, at) => drawing.bufferSubData(target, offset, new Uint8Array(memory.buffer, at, size)),
      glDeleteBuffers: glGone("buffer", (held) => drawing.deleteBuffer(held)),
      glGenVertexArrays: glFresh("array", () => drawing.createVertexArray()),
      glBindVertexArray: (array) => drawing.bindVertexArray(made.array[array] || null),
      glDeleteVertexArrays: glGone("array", (held) => drawing.deleteVertexArray(held)),
      glEnableVertexAttribArray: (index) => drawing.enableVertexAttribArray(index),
      glDisableVertexAttribArray: (index) => drawing.disableVertexAttribArray(index),
      glVertexAttribPointer: (index, size, type, normal, stride, offset) => drawing.vertexAttribPointer(index, size, type, !!normal, stride, offset),
      glVertexAttrib4f: (index, x, y, z, w) => drawing.vertexAttrib4f(index, x, y, z, w),
      glDrawArrays: (mode, first, count) => drawing.drawArrays(mode, first, count),
      glDrawElements: (mode, count, type, offset) => drawing.drawElements(mode, count, type, offset),

      glGenTextures: glFresh("texture", () => drawing.createTexture()),
      glBindTexture: (target, texture) => drawing.bindTexture(target, made.texture[texture] || null),
      glActiveTexture: (unit) => drawing.activeTexture(unit),
      glTexImage2D: (target, level, kind, width, height, border, format, type, at) =>
        drawing.texImage2D(target, level, kind, width, height, border, format, type, glDots(at, width, height, format, type)),
      glTexSubImage2D(target, level, x, y, width, height, format, type, at) {
        const dots = glDots(at, width, height, format, type);
        if (dots) drawing.texSubImage2D(target, level, x, y, width, height, format, type, dots);
      },
      glTexParameteri: (target, name, value) => drawing.texParameteri(target, name, value),
      glTexParameterf: (target, name, value) => drawing.texParameterf(target, name, value),
      glGenerateMipmap: (target) => drawing.generateMipmap(target),
      glDeleteTextures: glGone("texture", (held) => drawing.deleteTexture(held)),
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
          const answer = await fetch(network, { method: "POST", headers: { "Content-Type": "application/octet-stream" }, body: netBytes(going), cache: "no-store" });
          netOn = answer.ok ? 1 : -1;
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
        // The network's three calls, carried by GridCraft.
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
        for call in ["net_status", "net_send", "net_receive"] {
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
        // It is plain text a browser reads as it is: nothing a build step would have to make of it.
        assert!(LOADER.is_ascii() && !LOADER.contains('\r') && !LOADER.contains("import ") && !LOADER.contains("require("));
    }
}
