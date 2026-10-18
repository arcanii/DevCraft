//! The C Development Kit's showcase: a turning triangle, drawn by a C program with OpenGL ES.
//!
//! It shows what the kit's programs can draw with (the owner, 2026-10-14: OpenGL bindings "for
//! 3D games like Quake"; this is the first stage of it). The program is three files: a recipe,
//! one file of C that makes two shaders and three corners and draws them turned, and a page
//! with a canvas. The page names the canvas to the loader, and the loader answers the
//! program's OpenGL calls with it ([`crate::c_program`], [`crate::c::GL_HEADER`]).
//!
//! It is a project like any other: created untrusted, in the user's projects folder, and it
//! does nothing until the user trusts it and builds it. It asks to be allowed nothing.

use std::path::Path;

use crate::project::Project;
use crate::showcase::Showcase;

/// The showcase project's id, and its folder's name.
pub const ID: &str = "turning-triangle";
pub const NAME: &str = "Turning Triangle";

const MANIFEST: &str = r#"# The C Development Kit's showcase: a C program that draws with OpenGL ES.
id = "turning-triangle"
name = "Turning Triangle"
version = "0.1.0"
author = ""
description = "A C program that draws with OpenGL ES: a triangle, turning."

# The C Development Kit: GridCraft builds the C below as one WebAssembly program, and the
# panel runs it. It can reach nothing but what its page hands it: here, a canvas to draw on.
kit = "c"

# It asks for nothing: it neither sees nor changes a workbook.
permissions = []

[panel]
entry = "panel/index.html"

# Its page calls the two functions it exports (start, and frame for each picture): it has no main.
[c]
sources = ["src"]
entry = "none"
"#;
const SOURCE: &str = r##"/*
 * A turning triangle: a C program that draws with OpenGL ES, in a GridCraft panel.
 *
 * Built by the C Development Kit as WebAssembly. Its page names a canvas, and the kit's loader
 * answers the OpenGL calls below with that canvas. The page calls `start` once, and `frame`
 * for each picture, with how many seconds have gone by.
 */
#include <GLES3/gl3.h>
#include <math.h>
#include <stdio.h>

#define EXPORT(name) __attribute__((export_name(#name)))

static const char *const VERTEX =
    "#version 300 es\n"
    "in vec2 at;\n"
    "in vec3 colour;\n"
    "uniform mat4 turn;\n"
    "out vec3 shade;\n"
    "void main() {\n"
    "  shade = colour;\n"
    "  gl_Position = turn * vec4(at, 0.0, 1.0);\n"
    "}\n";

static const char *const FRAGMENT =
    "#version 300 es\n"
    "precision mediump float;\n"
    "in vec3 shade;\n"
    "out vec4 dot;\n"
    "void main() {\n"
    "  dot = vec4(shade, 1.0);\n"
    "}\n";

/* Three corners: where each is, and its colour. */
static const GLfloat CORNERS[] = {
    0.0f,  0.8f,  0.82f, 0.23f, 0.18f,
    -0.7f, -0.4f, 0.96f, 0.91f, 0.72f,
    0.7f,  -0.4f, 0.21f, 0.44f, 0.63f,
};

static GLuint program, corners, array;
static GLint turn;
static int ready;

static GLuint shader(GLenum kind, const char *text)
{
    GLuint made = glCreateShader(kind);
    GLint fine = 0;

    glShaderSource(made, 1, &text, NULL);
    glCompileShader(made);
    glGetShaderiv(made, GL_COMPILE_STATUS, &fine);
    if (!fine) {
        char said[512];
        glGetShaderInfoLog(made, sizeof said, NULL, said);
        printf("A shader was not made: %s\n", said);
    }
    return made;
}

/* Makes what is drawn with: the shaders, and the three corners. 1 when it is ready to draw. */
EXPORT(start) int start(void)
{
    GLint fine = 0;

    program = glCreateProgram();
    glAttachShader(program, shader(GL_VERTEX_SHADER, VERTEX));
    glAttachShader(program, shader(GL_FRAGMENT_SHADER, FRAGMENT));
    glLinkProgram(program);
    glGetProgramiv(program, GL_LINK_STATUS, &fine);
    if (!fine) {
        char said[512];
        glGetProgramInfoLog(program, sizeof said, NULL, said);
        printf("The shaders did not go together: %s\n", said);
        return 0;
    }
    turn = glGetUniformLocation(program, "turn");

    glGenVertexArrays(1, &array);
    glBindVertexArray(array);
    glGenBuffers(1, &corners);
    glBindBuffer(GL_ARRAY_BUFFER, corners);
    glBufferData(GL_ARRAY_BUFFER, sizeof CORNERS, CORNERS, GL_STATIC_DRAW);
    {
        GLint at = glGetAttribLocation(program, "at");
        GLint colour = glGetAttribLocation(program, "colour");
        glEnableVertexAttribArray((GLuint) at);
        glVertexAttribPointer((GLuint) at, 2, GL_FLOAT, GL_FALSE, 5 * sizeof(GLfloat), (const void *) 0);
        glEnableVertexAttribArray((GLuint) colour);
        glVertexAttribPointer((GLuint) colour, 3, GL_FLOAT, GL_FALSE, 5 * sizeof(GLfloat), (const void *) (2 * sizeof(GLfloat)));
    }
    ready = glGetError() == GL_NO_ERROR;
    printf(ready ? "Ready to draw.\n" : "OpenGL said something went wrong.\n");
    return ready;
}

/* One picture, `seconds` after the start, on a canvas `width` by `height`. */
EXPORT(frame) void frame(double seconds, int width, int height)
{
    float c = cosf((float) seconds), s = sinf((float) seconds);
    float wide = height > 0 ? (float) width / (float) height : 1.0f;
    /* Turned about the middle, and made no wider than it is tall. */
    GLfloat matrix[16] = {
        c / wide, s, 0, 0,
        -s / wide, c, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1,
    };

    if (!ready)
        return;
    glViewport(0, 0, width, height);
    glClearColor(0.09f, 0.10f, 0.11f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT);
    glUseProgram(program);
    glUniformMatrix4fv(turn, 1, GL_FALSE, matrix);
    glBindVertexArray(array);
    glDrawArrays(GL_TRIANGLES, 0, 3);
}
"##;
const PANEL: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Turning Triangle</title>
  <style>
    html, body { margin: 0; background: #16181c; color: #d8dbe0; font: 13px system-ui, sans-serif; }
    body { padding: 10px; }
    #picture { display: block; width: 100%; aspect-ratio: 4 / 3; background: #000; border: 2px solid #2a2d33; box-sizing: border-box; }
    p { margin: 8px 0 0; }
    pre { background: #0e0f12; padding: 8px; font-size: 11px; white-space: pre-wrap; margin: 8px 0 0; }
  </style>
</head>
<body>
  <canvas id="picture" aria-label="A turning triangle"></canvas>
  <p id="says" aria-live="polite"></p>
  <pre id="lines"></pre>

  <!-- GridCraft serves these two beside this page: neither is a file of the project. -->
  <script src="program.js"></script>
  <script>
    const picture = document.getElementById("picture");
    const says = document.getElementById("says");
    const lines = document.getElementById("lines");

    async function page() {
      // The program draws on this canvas, and on nothing else: the loader answers its OpenGL
      // calls with the canvas's own.
      const program = await CProgram.load({ canvas: picture, print: (line) => (lines.textContent += line + "\n") });
      if (!program.draws) {
        says.textContent = "This page can't draw with OpenGL here.";
        return;
      }
      if (!program.exports.start()) {
        says.textContent = "The program could not get ready to draw: what it printed is below.";
        return;
      }
      says.textContent = "Drawn by a C program, with OpenGL ES.";
      const began = performance.now();
      const frame = () => {
        if (program.ended()) {
          says.textContent = program.ended();
          return;
        }
        // The canvas is as many dots as it is shown with.
        const [width, height] = [picture.clientWidth * devicePixelRatio, picture.clientHeight * devicePixelRatio].map(Math.round);
        if (picture.width !== width || picture.height !== height) [picture.width, picture.height] = [width, height];
        program.exports.frame((performance.now() - began) / 1000, width, height);
        requestAnimationFrame(frame);
      };
      requestAnimationFrame(frame);
    }

    page().catch((error) => (says.textContent = String(error.message || error)));
  </script>
</body>
</html>
"#;

/// The showcase's files: where each goes in the project, and its text.
pub const FILES: [(&str, &str); 3] = [("project.toml", MANIFEST), ("src/triangle.c", SOURCE), ("panel/index.html", PANEL)];

/// The showcase, as the C kit's part offers it.
pub const SHOWCASE: Showcase =
    Showcase { id: ID, name: NAME, kit: crate::kit::C.id, says: "A C program that draws with OpenGL ES: a triangle, turning.", files: &FILES };

/// Creates the showcase project in `projects_dir` and returns it as read back from disk. It
/// never touches a folder that already exists.
pub fn create(projects_dir: &Path) -> Result<Project, String> {
    SHOWCASE.create(projects_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::tests::TempDir;
    use crate::trust::{Status, TrustStore};

    #[test]
    fn the_c_showcase_is_a_sound_project_that_asks_for_nothing_and_starts_untrusted() {
        let t = TempDir::new("triangle");
        let made = create(&t.0).unwrap();
        assert!(made.is_sound(), "{:?}", made.problem);
        assert_eq!((made.id.as_str(), made.name()), (ID, NAME));
        let manifest = made.manifest.as_ref().unwrap();
        assert_eq!((manifest.kit.as_str(), manifest.permissions.len(), manifest.network.as_ref()), ("c", 0, None));
        let recipe = manifest.c.as_ref().unwrap();
        assert_eq!(
            (recipe.entry, recipe.setjmp, recipe.sources_among(&made.files).unwrap()),
            (crate::c::Entry::None, false, vec!["src/triangle.c".to_string()])
        );
        assert_eq!(made.files, ["panel/index.html", "project.toml", "src/triangle.c"]);
        assert_eq!(TrustStore::load(&t.0.join("trusted.json")).status(&made), Status::Untrusted);
        // It draws with the kit's header, on the canvas its page names, and asks the sheet nothing.
        assert!(SOURCE.contains("#include <GLES3/gl3.h>") && SOURCE.contains("glDrawArrays"));
        assert!(PANEL.contains("CProgram.load({ canvas: picture") && !PANEL.contains("spreadsheet"));
        // Every call it makes is one the kit's header declares.
        for call in SOURCE
            .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .filter(|word| word.starts_with("gl") && word[2..].starts_with(char::is_uppercase))
        {
            assert!(crate::c::GL_HEADER.contains(&format!("GRIDCRAFT_GL({call})")), "{call}");
        }
        // A second time, the folder is there: nothing is written over.
        assert!(create(&t.0).unwrap_err().contains("already a project folder"));
    }
}
