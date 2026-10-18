//! OpenGL 1 for a program of the C Development Kit: `<GL/gl.h>`, made of the OpenGL ES 3 the
//! kit hands a program (the owner, 2026-10-14: "the kit should offer OpenGL itself as a regular
//! devkit feature", after GLQuake was drawn with a layer of its own).
//!
//! The kit hands a program OpenGL ES 3 ([`crate::c::GL_HEADER`]), which is what a page has.
//! Games of the nineties were written for OpenGL 1: they draw a shape a corner at a time
//! (`glBegin`, `glVertex`, `glEnd`), keep stacks of matrices, and colour what they draw by a
//! fixed recipe (`glTexEnv`, `glAlphaFunc`). OpenGL ES 3 has none of that. [`SOURCE`] is C that
//! does it on top of OpenGL ES 3:
//!
//! - the corners of a shape are turned into triangles as they come, each put through the
//!   modelview matrix there, with its place in the texture and its colour;
//! - the triangles are gathered and drawn many at a time: whenever something that changes how
//!   they are drawn changes (a texture, blending, the projection), what was gathered is drawn
//!   first. GLQuake draws a picture so in a few dozen draws;
//! - one shader does the fixed recipe: the projection, the texture environment (modulate,
//!   replace, decal, blend) for each kind of texture, and the alpha test;
//! - OpenGL 1 lets a program name its textures itself, OpenGL ES 3 names them: a table holds both.
//!
//! What it does not have: lighting, fog, texture coordinates made by OpenGL, arrays of corners,
//! display lists, more than one texture at a time, points. And of GLU, `gluPerspective`,
//! `gluOrtho2D` and `gluLookAt`.
//!
//! **A program that includes `<GL/gl.h>` (or `<GL/glu.h>`) is built with it**; one that does
//! not is built without it ([`crate::c`] sees which from what the compiler says each source
//! read). A project's own header of the same name wins, as with OpenGL ES's: then the kit's
//! OpenGL 1 is neither read nor built in. It draws on the canvas the page names, through the
//! loader's OpenGL ES 3, and gains the program nothing more.
//!
//! The calls OpenGL 1 shares a name with OpenGL ES 3 are renamed in the header (`glEnable` is
//! `gl1_Enable`), so that a program that includes it calls OpenGL 1, and OpenGL 1 calls OpenGL
//! ES 3. Text inside Rust source, as the loader is: the repository holds no C file.

/// `<GL/gl.h>`.
pub const HEADER: &str = r##"/*
 * GridCraft's C Development Kit: OpenGL 1, the part of it games of the nineties draw with,
 * made of the OpenGL ES 3 the kit hands a program (<GLES3/gl3.h>).
 *
 * A program that includes this is built with the kit's OpenGL 1 (gl1.c, the kit's own); one
 * that does not is built without it. It draws on the canvas the program's page names, as
 * OpenGL ES 3 does. Not here: lighting, fog, texture coordinates made by OpenGL, arrays of
 * corners, display lists, more than one texture at a time, points.
 *
 * The calls OpenGL 1 shares a name with OpenGL ES 3 are renamed below (glEnable is
 * gl1_Enable), so that a program that includes this calls OpenGL 1, and OpenGL 1 calls OpenGL
 * ES 3. The names and numbers are OpenGL's own; the file is GridCraft's, written for this kit,
 * and is not a project's to change: GridCraft supplies it when it builds.
 */
#ifndef GRIDCRAFT_GL1_GL_H
#define GRIDCRAFT_GL1_GL_H

#include <GLES3/gl3.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef double GLdouble;
typedef double GLclampd;

/* Numbers OpenGL 1 has that OpenGL ES 3 does not. */
#define GL_QUADS 0x0007
#define GL_QUAD_STRIP 0x0008
#define GL_POLYGON 0x0009
#define GL_FRONT_LEFT 0x0400
#define GL_BACK_LEFT 0x0402
#define GL_FOG 0x0B60
#define GL_FOG_DENSITY 0x0B62
#define GL_FOG_START 0x0B63
#define GL_FOG_END 0x0B64
#define GL_FOG_MODE 0x0B65
#define GL_FOG_COLOR 0x0B66
#define GL_MODELVIEW_MATRIX 0x0BA6
#define GL_PROJECTION_MATRIX 0x0BA7
#define GL_ALPHA_TEST 0x0BC0
#define GL_PERSPECTIVE_CORRECTION_HINT 0x0C50
#define GL_COLOR_INDEX 0x1900
#define GL_MODELVIEW 0x1700
#define GL_PROJECTION 0x1701
#define GL_POINT 0x1B00
#define GL_LINE 0x1B01
#define GL_FILL 0x1B02
#define GL_FLAT 0x1D00
#define GL_SMOOTH 0x1D01
#define GL_MODULATE 0x2100
#define GL_DECAL 0x2101
#define GL_TEXTURE_ENV_MODE 0x2200
#define GL_TEXTURE_ENV_COLOR 0x2201
#define GL_TEXTURE_ENV 0x2300
#define GL_CLAMP 0x2900
#define GL_ALPHA8 0x803C
#define GL_LUMINANCE8 0x8040
#define GL_LUMINANCE8_ALPHA8 0x8045
#define GL_INTENSITY 0x8049
#define GL_INTENSITY8 0x804B
#define GL_RGB5 0x8050
#define GL_VERTEX_ARRAY_EXT 0x8074
#define GL_TEXTURE_COORD_ARRAY_EXT 0x8078
#define GL_SHARED_TEXTURE_PALETTE_EXT 0x81FB

/* The calls OpenGL 1 shares with OpenGL ES 3, by the names the kit's OpenGL 1 answers them under. */
#ifndef GRIDCRAFT_GL1_INSIDE
#define glViewport gl1_Viewport
#define glScissor gl1_Scissor
#define glClear gl1_Clear
#define glClearColor gl1_ClearColor
#define glEnable gl1_Enable
#define glDisable gl1_Disable
#define glDepthFunc gl1_DepthFunc
#define glDepthMask gl1_DepthMask
#define glCullFace gl1_CullFace
#define glFrontFace gl1_FrontFace
#define glBlendFunc gl1_BlendFunc
#define glColorMask gl1_ColorMask
#define glPolygonOffset gl1_PolygonOffset
#define glLineWidth gl1_LineWidth
#define glHint gl1_Hint
#define glPixelStorei gl1_PixelStorei
#define glReadPixels gl1_ReadPixels
#define glReadBuffer gl1_ReadBuffer
#define glFinish gl1_Finish
#define glFlush gl1_Flush
#define glGetError gl1_GetError
#define glGetString gl1_GetString
#define glGetFloatv gl1_GetFloatv
#define glGetIntegerv gl1_GetIntegerv
#define glGenTextures gl1_GenTextures
#define glDeleteTextures gl1_DeleteTextures
#define glBindTexture gl1_BindTexture
#define glTexImage2D gl1_TexImage2D
#define glTexSubImage2D gl1_TexSubImage2D
#define glTexParameteri gl1_TexParameteri
#define glTexParameterf gl1_TexParameterf
#endif

void gl1_Viewport(GLint x, GLint y, GLsizei width, GLsizei height);
void gl1_Scissor(GLint x, GLint y, GLsizei width, GLsizei height);
void gl1_Clear(GLbitfield mask);
void gl1_ClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha);
void gl1_Enable(GLenum cap);
void gl1_Disable(GLenum cap);
void gl1_DepthFunc(GLenum func);
void gl1_DepthMask(GLboolean flag);
void gl1_CullFace(GLenum mode);
void gl1_FrontFace(GLenum mode);
void gl1_BlendFunc(GLenum sfactor, GLenum dfactor);
void gl1_ColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
void gl1_PolygonOffset(GLfloat factor, GLfloat units);
void gl1_LineWidth(GLfloat width);
void gl1_Hint(GLenum target, GLenum mode);
void gl1_PixelStorei(GLenum pname, GLint param);
void gl1_ReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid *pixels);
void gl1_ReadBuffer(GLenum mode);
void gl1_Finish(void);
void gl1_Flush(void);
GLenum gl1_GetError(void);
const GLubyte *gl1_GetString(GLenum name);
void gl1_GetFloatv(GLenum pname, GLfloat *params);
void gl1_GetIntegerv(GLenum pname, GLint *params);
void gl1_GenTextures(GLsizei n, GLuint *textures);
void gl1_DeleteTextures(GLsizei n, const GLuint *textures);
void gl1_BindTexture(GLenum target, GLuint texture);
void gl1_TexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const GLvoid *pixels);
void gl1_TexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const GLvoid *pixels);
void gl1_TexParameteri(GLenum target, GLenum pname, GLint param);
void gl1_TexParameterf(GLenum target, GLenum pname, GLfloat param);

/* OpenGL 1's own. */
void glBegin(GLenum mode);
void glEnd(void);
void glVertex2f(GLfloat x, GLfloat y);
void glVertex2i(GLint x, GLint y);
void glVertex3f(GLfloat x, GLfloat y, GLfloat z);
void glVertex3fv(const GLfloat *v);
void glTexCoord2f(GLfloat s, GLfloat t);
void glTexCoord2fv(const GLfloat *v);
void glColor3f(GLfloat red, GLfloat green, GLfloat blue);
void glColor3fv(const GLfloat *v);
void glColor4f(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
void glColor4fv(const GLfloat *v);
void glColor3ub(GLubyte red, GLubyte green, GLubyte blue);
void glColor3ubv(const GLubyte *v);
void glColor4ub(GLubyte red, GLubyte green, GLubyte blue, GLubyte alpha);
void glColor4ubv(const GLubyte *v);

void glMatrixMode(GLenum mode);
void glLoadIdentity(void);
void glLoadMatrixf(const GLfloat *m);
void glMultMatrixf(const GLfloat *m);
void glPushMatrix(void);
void glPopMatrix(void);
void glRotatef(GLfloat angle, GLfloat x, GLfloat y, GLfloat z);
void glTranslatef(GLfloat x, GLfloat y, GLfloat z);
void glScalef(GLfloat x, GLfloat y, GLfloat z);
void glFrustum(GLdouble left, GLdouble right, GLdouble bottom, GLdouble top, GLdouble zNear, GLdouble zFar);
void glOrtho(GLdouble left, GLdouble right, GLdouble bottom, GLdouble top, GLdouble zNear, GLdouble zFar);

void glTexEnvf(GLenum target, GLenum pname, GLfloat param);
void glTexEnvi(GLenum target, GLenum pname, GLint param);
void glTexEnvfv(GLenum target, GLenum pname, const GLfloat *params);
void glAlphaFunc(GLenum func, GLclampf ref);
void glDepthRange(GLclampd zNear, GLclampd zFar);
void glClearDepth(GLclampd depth);
void glShadeModel(GLenum mode);
void glPolygonMode(GLenum face, GLenum mode);
void glDrawBuffer(GLenum mode);
void glFogf(GLenum pname, GLfloat param);
void glFogi(GLenum pname, GLint param);
void glFogfv(GLenum pname, const GLfloat *params);

#ifdef __cplusplus
}
#endif

#endif
"##;

/// `<GL/glu.h>`: the three of GLU that set up a view.
pub const GLU_HEADER: &str = r##"/*
 * GridCraft's C Development Kit: the three calls of GLU that set up a view, made by the kit's
 * OpenGL 1 (<GL/gl.h>, which this includes). The file is GridCraft's, written for this kit.
 */
#ifndef GRIDCRAFT_GL1_GLU_H
#define GRIDCRAFT_GL1_GLU_H

#include <GL/gl.h>

#ifdef __cplusplus
extern "C" {
#endif

void gluPerspective(GLdouble fovy, GLdouble aspect, GLdouble zNear, GLdouble zFar);
void gluOrtho2D(GLdouble left, GLdouble right, GLdouble bottom, GLdouble top);
void gluLookAt(GLdouble eyeX, GLdouble eyeY, GLdouble eyeZ, GLdouble centerX, GLdouble centerY, GLdouble centerZ, GLdouble upX,
               GLdouble upY, GLdouble upZ);

#ifdef __cplusplus
}
#endif

#endif
"##;

/// The kit's OpenGL 1, built into a program that includes [`HEADER`].
pub const SOURCE: &str = r##"/*
 * GridCraft's C Development Kit: OpenGL 1, made of the OpenGL ES 3 the kit hands a program.
 * Built into a program that includes <GL/gl.h>. The file is GridCraft's, written for this kit.
 *
 * OpenGL 1 draws a shape a corner at a time (glBegin, glVertex, glEnd), keeps stacks of
 * matrices, and colours what it draws by a fixed recipe (glTexEnv, glAlphaFunc). OpenGL ES 3
 * has none of that: it draws buffers of corners, with shaders. So here:
 *
 *   - The corners of a shape are turned into triangles as they come, each put through the
 *     modelview matrix here, with its place in the texture and its colour.
 *   - The triangles are gathered and drawn many at a time. Whenever something that changes how
 *     they are drawn changes (a texture, blending, the projection), what was gathered is drawn
 *     first.
 *   - One shader does OpenGL 1's fixed recipe: the projection, the texture environment
 *     (modulate, replace, decal, blend) for each kind of texture, and the alpha test.
 *   - OpenGL 1 lets a program name its textures itself; OpenGL ES 3 names them. A table here
 *     holds the program's names beside the ones OpenGL ES 3 gave.
 */
#define GRIDCRAFT_GL1_INSIDE
#include <GL/gl.h>
#include <GL/glu.h>

#include <math.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* ---- the state ---------------------------------------------------------------------------- */

/* A corner as it is drawn: where (after the modelview matrix), its place in the texture, its colour. */
typedef struct {
    GLfloat at[3];
    GLfloat st[2];
    GLubyte colour[4];
} corner;

/* What is gathered to be drawn: a multiple of six corners. */
#define GATHERED 49152
static corner gathered[GATHERED];
static int gathered_count;
static GLenum gathered_mode = GL_TRIANGLES;

/* The shape being given a corner at a time, and the corners it still needs. */
#define NO_SHAPE 0xFFFFu
static GLenum shape = NO_SHAPE;
static int shape_corners;
static corner held[4];
static GLfloat now_st[2];
static GLubyte now_colour[4] = {255, 255, 255, 255};

/* The matrices: two stacks, as OpenGL 1 keeps them. */
#define DEPTH 32
static GLfloat modelview[DEPTH][16];
static GLfloat projection[DEPTH][16];
static int modelview_top, projection_top;
static GLenum matrix_mode = GL_MODELVIEW;
static int projection_changed = 1;

/* The fixed recipe. */
static int texturing;
static GLenum env_mode = GL_MODULATE;
static GLfloat env_colour[4];
static int alpha_testing;
static GLenum alpha_func = GL_ALWAYS;
static GLfloat alpha_reference;

/* What a texture gives, by the kind OpenGL 1 was told it is: colour and alpha, colour only,
   alpha only (held in its red), or both from one value (held in its red). */
enum { TAKES_BOTH, TAKES_COLOUR, TAKES_ALPHA, TAKES_ONE };

/* The program's names for its textures, and what OpenGL ES 3 holds for each. */
#define NAMES 8192
static struct {
    GLuint real;
    GLenum uploaded; /* the format it is held in: GL_RGBA, GL_RGB or GL_LUMINANCE */
    GLubyte takes;
    GLubyte given;   /* handed out by glGenTextures */
} names[NAMES];
static GLuint bound;

/* What OpenGL ES 3 is set to, so that what does not change is not drawn for. */
static struct {
    int blend, depth_test, cull_face, scissor_test, polygon_offset, stencil_test;
    GLenum depth_func, cull_mode, front_face, blend_from, blend_to;
    GLboolean depth_mask, mask[4];
    GLfloat depth_near, depth_far, offset_factor, offset_units, line_width;
} es = {0, 0, 0, 0, 0, 0, GL_LESS, GL_BACK, GL_CCW, GL_ONE, GL_ZERO, GL_TRUE, {GL_TRUE, GL_TRUE, GL_TRUE, GL_TRUE}, 0, 1, 0, 0, 1};
static GLint unpack_alignment = 4, pack_alignment = 4;

/* The shader, and what it is told. */
static GLuint program, buffer, array;
static GLint u_projection, u_picture, u_how, u_takes, u_env, u_test, u_reference;
static struct {
    int how, takes, test;
    GLfloat reference, env[4];
} told = {-1, -1, -1, -1, {-1, -1, -1, -1}};

static const char *const VERTEX =
    "#version 300 es\n"
    "layout(location = 0) in vec3 at;\n"
    "layout(location = 1) in vec2 st;\n"
    "layout(location = 2) in vec4 colour;\n"
    "uniform mat4 projection;\n"
    "out vec2 v_st;\n"
    "out vec4 v_colour;\n"
    "void main() {\n"
    "    gl_Position = projection * vec4(at, 1.0);\n"
    "    v_st = st;\n"
    "    v_colour = colour;\n"
    "}\n";

/* how: 0 no texture, 1 modulate, 2 replace, 3 decal, 4 blend. takes: as TAKES_ above.
   test: 0 none, else the alpha test's function less GL_NEVER, plus one. */
static const char *const FRAGMENT =
    "#version 300 es\n"
    "precision highp float;\n"
    "in vec2 v_st;\n"
    "in vec4 v_colour;\n"
    "uniform sampler2D picture;\n"
    "uniform int how;\n"
    "uniform int takes;\n"
    "uniform vec4 env;\n"
    "uniform int test;\n"
    "uniform float reference;\n"
    "out vec4 fragment;\n"
    "void main() {\n"
    "    vec4 c = v_colour;\n"
    "    if (how != 0) {\n"
    "        vec4 t = texture(picture, v_st);\n"
    "        bool colour = takes != 2;\n"
    "        bool alpha = takes != 1;\n"
    "        vec3 tc = takes == 3 ? vec3(t.r) : t.rgb;\n"
    "        float ta = takes >= 2 ? t.r : t.a;\n"
    "        if (how == 1) {\n"
    "            if (colour) c.rgb *= tc;\n"
    "            if (alpha) c.a *= ta;\n"
    "        } else if (how == 2) {\n"
    "            if (colour) c.rgb = tc;\n"
    "            if (alpha) c.a = ta;\n"
    "        } else if (how == 3) {\n"
    "            c.rgb = alpha ? mix(c.rgb, tc, ta) : tc;\n"
    "        } else {\n"
    "            if (colour) c.rgb = mix(c.rgb, env.rgb, tc);\n"
    "            if (alpha) c.a *= ta;\n"
    "        }\n"
    "    }\n"
    "    if (test != 0) {\n"
    "        float a = clamp(c.a, 0.0, 1.0);\n"
    "        bool pass = test == 2 ? a < reference : test == 3 ? a == reference : test == 4 ? a <= reference\n"
    "            : test == 5 ? a > reference : test == 6 ? a != reference : test == 7 ? a >= reference : test == 8;\n"
    "        if (!pass) discard;\n"
    "    }\n"
    "    fragment = c;\n"
    "}\n";

/* ---- starting ----------------------------------------------------------------------------- */

static void identity(GLfloat *m)
{
    memset(m, 0, 16 * sizeof *m);
    m[0] = m[5] = m[10] = m[15] = 1;
}

static GLuint shader(GLenum kind, const char *source)
{
    GLuint made = glCreateShader(kind);
    GLint compiled = 0;

    glShaderSource(made, 1, &source, NULL);
    glCompileShader(made);
    glGetShaderiv(made, GL_COMPILE_STATUS, &compiled);
    if (!compiled) {
        char log[1024];
        glGetShaderInfoLog(made, sizeof log, NULL, log);
        printf("OpenGL 1: a shader did not compile: %s\n", log);
    }
    return made;
}

/* The texture the program calls `name`, made in OpenGL ES 3 the first time it is used. */
static GLuint real_texture(GLuint name)
{
    if (!names[name].real) {
        glGenTextures(1, &names[name].real);
        names[name].uploaded = GL_RGBA;
        names[name].takes = TAKES_BOTH;
    }
    return names[name].real;
}

static int started;

static void start(void)
{
    GLuint vertex, fragment;
    GLint linked = 0;

    started = 1;
    identity(modelview[0]);
    identity(projection[0]);

    vertex = shader(GL_VERTEX_SHADER, VERTEX);
    fragment = shader(GL_FRAGMENT_SHADER, FRAGMENT);
    program = glCreateProgram();
    glAttachShader(program, vertex);
    glAttachShader(program, fragment);
    glLinkProgram(program);
    glGetProgramiv(program, GL_LINK_STATUS, &linked);
    if (!linked) {
        char log[1024];
        glGetProgramInfoLog(program, sizeof log, NULL, log);
        printf("OpenGL 1: the shader did not link: %s\n", log);
    }
    glDeleteShader(vertex);
    glDeleteShader(fragment);
    glUseProgram(program);
    u_projection = glGetUniformLocation(program, "projection");
    u_picture = glGetUniformLocation(program, "picture");
    u_how = glGetUniformLocation(program, "how");
    u_takes = glGetUniformLocation(program, "takes");
    u_env = glGetUniformLocation(program, "env");
    u_test = glGetUniformLocation(program, "test");
    u_reference = glGetUniformLocation(program, "reference");
    glUniform1i(u_picture, 0);

    glGenVertexArrays(1, &array);
    glBindVertexArray(array);
    glGenBuffers(1, &buffer);
    glBindBuffer(GL_ARRAY_BUFFER, buffer);
    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, sizeof(corner), (const void *) offsetof(corner, at));
    glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, sizeof(corner), (const void *) offsetof(corner, st));
    glVertexAttribPointer(2, 4, GL_UNSIGNED_BYTE, GL_TRUE, sizeof(corner), (const void *) offsetof(corner, colour));
    glEnableVertexAttribArray(0);
    glEnableVertexAttribArray(1);
    glEnableVertexAttribArray(2);

    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, real_texture(0));
}

#define ONCE()           \
    do {                 \
        if (!started)    \
            start();     \
    } while (0)

/* ---- drawing what was gathered ------------------------------------------------------------ */

static int how_now(void)
{
    if (!texturing)
        return 0;
    switch (env_mode) {
    case GL_REPLACE: return 2;
    case GL_DECAL: return 3;
    case GL_BLEND: return 4;
    default: return 1;
    }
}

static void flush(void)
{
    int how, takes, test;

    if (!gathered_count)
        return;
    if (projection_changed) {
        glUniformMatrix4fv(u_projection, 1, GL_FALSE, projection[projection_top]);
        projection_changed = 0;
    }
    how = how_now();
    if (how != told.how)
        glUniform1i(u_how, told.how = how);
    takes = names[bound].takes;
    if (how && takes != told.takes)
        glUniform1i(u_takes, told.takes = takes);
    if (how == 4 && memcmp(env_colour, told.env, sizeof env_colour)) {
        memcpy(told.env, env_colour, sizeof env_colour);
        glUniform4fv(u_env, 1, env_colour);
    }
    test = alpha_testing && alpha_func != GL_ALWAYS ? (int) (alpha_func - GL_NEVER) + 1 : 0;
    if (test != told.test)
        glUniform1i(u_test, told.test = test);
    if (test && alpha_reference != told.reference)
        glUniform1f(u_reference, told.reference = alpha_reference);
    glBufferData(GL_ARRAY_BUFFER, (GLsizeiptr) (gathered_count * sizeof(corner)), gathered, GL_STREAM_DRAW);
    glDrawArrays(gathered_mode, 0, gathered_count);
    gathered_count = 0;
}

/* ---- shapes, a corner at a time ----------------------------------------------------------- */

static void room_for(int corners, GLenum mode)
{
    if (gathered_mode != mode || gathered_count + corners > GATHERED) {
        flush();
        gathered_mode = mode;
    }
}

static void triangle(const corner *a, const corner *b, const corner *c)
{
    room_for(3, GL_TRIANGLES);
    gathered[gathered_count++] = *a;
    gathered[gathered_count++] = *b;
    gathered[gathered_count++] = *c;
}

static void line(const corner *a, const corner *b)
{
    room_for(2, GL_LINES);
    gathered[gathered_count++] = *a;
    gathered[gathered_count++] = *b;
}

void glBegin(GLenum mode)
{
    ONCE();
    shape = mode;
    shape_corners = 0;
}

void glEnd(void)
{
    if (shape == GL_LINE_LOOP && shape_corners > 2)
        line(&held[1], &held[0]);
    shape = NO_SHAPE;
}

void glVertex3f(GLfloat x, GLfloat y, GLfloat z)
{
    const GLfloat *m;
    corner c;
    int n = shape_corners;

    if (shape == NO_SHAPE)
        return;
    m = modelview[modelview_top];
    c.at[0] = m[0] * x + m[4] * y + m[8] * z + m[12];
    c.at[1] = m[1] * x + m[5] * y + m[9] * z + m[13];
    c.at[2] = m[2] * x + m[6] * y + m[10] * z + m[14];
    c.st[0] = now_st[0];
    c.st[1] = now_st[1];
    memcpy(c.colour, now_colour, sizeof c.colour);

    switch (shape) {
    case GL_TRIANGLES:
        held[n % 3] = c;
        if (n % 3 == 2)
            triangle(&held[0], &held[1], &held[2]);
        break;
    case GL_QUADS:
        held[n % 4] = c;
        if (n % 4 == 3) {
            triangle(&held[0], &held[1], &held[2]);
            triangle(&held[0], &held[2], &held[3]);
        }
        break;
    case GL_TRIANGLE_FAN:
    case GL_POLYGON:
        if (n < 2) {
            held[n] = c;
        } else {
            triangle(&held[0], &held[1], &c);
            held[1] = c;
        }
        break;
    case GL_TRIANGLE_STRIP:
        if (n < 2) {
            held[n] = c;
        } else {
            /* Every other triangle of a strip is turned, so that each faces the way the first does. */
            if (n % 2)
                triangle(&held[1], &held[0], &c);
            else
                triangle(&held[0], &held[1], &c);
            held[0] = held[1];
            held[1] = c;
        }
        break;
    case GL_QUAD_STRIP:
        if (n < 2) {
            held[n] = c;
        } else if (n % 2 == 0) {
            held[2] = c;
        } else {
            triangle(&held[0], &held[1], &c);
            triangle(&held[0], &c, &held[2]);
            held[0] = held[2];
            held[1] = c;
        }
        break;
    case GL_LINES:
        held[n % 2] = c;
        if (n % 2)
            line(&held[0], &held[1]);
        break;
    case GL_LINE_STRIP:
    case GL_LINE_LOOP:
        if (n == 0) {
            held[0] = held[1] = c;
        } else {
            line(&held[1], &c);
            held[1] = c;
        }
        break;
    default:
        break; /* points, and anything else: not drawn */
    }
    shape_corners = n + 1;
}

void glVertex2f(GLfloat x, GLfloat y)
{
    glVertex3f(x, y, 0);
}

void glVertex2i(GLint x, GLint y)
{
    glVertex3f((GLfloat) x, (GLfloat) y, 0);
}

void glVertex3fv(const GLfloat *v)
{
    glVertex3f(v[0], v[1], v[2]);
}

void glTexCoord2f(GLfloat s, GLfloat t)
{
    now_st[0] = s;
    now_st[1] = t;
}

void glTexCoord2fv(const GLfloat *v)
{
    glTexCoord2f(v[0], v[1]);
}

static GLubyte byte_of(GLfloat value)
{
    if (!(value > 0))
        return 0;
    if (value >= 1)
        return 255;
    return (GLubyte) (value * 255 + 0.5f);
}

void glColor4f(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha)
{
    now_colour[0] = byte_of(red);
    now_colour[1] = byte_of(green);
    now_colour[2] = byte_of(blue);
    now_colour[3] = byte_of(alpha);
}

void glColor3f(GLfloat red, GLfloat green, GLfloat blue)
{
    glColor4f(red, green, blue, 1);
}

void glColor3fv(const GLfloat *v)
{
    glColor4f(v[0], v[1], v[2], 1);
}

void glColor4fv(const GLfloat *v)
{
    glColor4f(v[0], v[1], v[2], v[3]);
}

void glColor4ub(GLubyte red, GLubyte green, GLubyte blue, GLubyte alpha)
{
    now_colour[0] = red;
    now_colour[1] = green;
    now_colour[2] = blue;
    now_colour[3] = alpha;
}

void glColor3ub(GLubyte red, GLubyte green, GLubyte blue)
{
    glColor4ub(red, green, blue, 255);
}

void glColor3ubv(const GLubyte *v)
{
    glColor4ub(v[0], v[1], v[2], 255);
}

void glColor4ubv(const GLubyte *v)
{
    glColor4ub(v[0], v[1], v[2], v[3]);
}

/* ---- matrices ----------------------------------------------------------------------------- */

static GLfloat *matrix(void)
{
    return matrix_mode == GL_PROJECTION ? projection[projection_top] : modelview[modelview_top];
}

/* The projection is the shader's: what was gathered under the one it is now is drawn first. */
static void changing(void)
{
    ONCE();
    if (matrix_mode == GL_PROJECTION) {
        flush();
        projection_changed = 1;
    }
}

/* m = m times n, as OpenGL multiplies the matrix it is working on. */
static void multiply(GLfloat *m, const GLfloat *n)
{
    GLfloat out[16];
    int row, column;

    for (column = 0; column < 4; column++)
        for (row = 0; row < 4; row++)
            out[column * 4 + row] = m[0 * 4 + row] * n[column * 4 + 0] + m[1 * 4 + row] * n[column * 4 + 1] +
                                    m[2 * 4 + row] * n[column * 4 + 2] + m[3 * 4 + row] * n[column * 4 + 3];
    memcpy(m, out, sizeof out);
}

void glMatrixMode(GLenum mode)
{
    ONCE();
    if (mode == GL_MODELVIEW || mode == GL_PROJECTION)
        matrix_mode = mode;
}

void glLoadIdentity(void)
{
    changing();
    identity(matrix());
}

void glLoadMatrixf(const GLfloat *m)
{
    changing();
    memcpy(matrix(), m, 16 * sizeof *m);
}

void glMultMatrixf(const GLfloat *m)
{
    changing();
    multiply(matrix(), m);
}

void glPushMatrix(void)
{
    ONCE();
    if (matrix_mode == GL_PROJECTION) {
        if (projection_top + 1 < DEPTH) {
            memcpy(projection[projection_top + 1], projection[projection_top], sizeof projection[0]);
            projection_top++;
        }
    } else if (modelview_top + 1 < DEPTH) {
        memcpy(modelview[modelview_top + 1], modelview[modelview_top], sizeof modelview[0]);
        modelview_top++;
    }
}

void glPopMatrix(void)
{
    changing();
    if (matrix_mode == GL_PROJECTION) {
        if (projection_top > 0)
            projection_top--;
    } else if (modelview_top > 0) {
        modelview_top--;
    }
}

void glRotatef(GLfloat angle, GLfloat x, GLfloat y, GLfloat z)
{
    GLfloat r[16], length = sqrtf(x * x + y * y + z * z), c, s, t;
    double radians = angle * 3.14159265358979323846 / 180.0;

    changing();
    if (length == 0)
        return;
    x /= length;
    y /= length;
    z /= length;
    c = (GLfloat) cos(radians);
    s = (GLfloat) sin(radians);
    t = 1 - c;
    identity(r);
    r[0] = x * x * t + c;
    r[1] = y * x * t + z * s;
    r[2] = x * z * t - y * s;
    r[4] = x * y * t - z * s;
    r[5] = y * y * t + c;
    r[6] = y * z * t + x * s;
    r[8] = x * z * t + y * s;
    r[9] = y * z * t - x * s;
    r[10] = z * z * t + c;
    multiply(matrix(), r);
}

void glTranslatef(GLfloat x, GLfloat y, GLfloat z)
{
    GLfloat t[16];

    changing();
    identity(t);
    t[12] = x;
    t[13] = y;
    t[14] = z;
    multiply(matrix(), t);
}

void glScalef(GLfloat x, GLfloat y, GLfloat z)
{
    GLfloat s[16];

    changing();
    identity(s);
    s[0] = x;
    s[5] = y;
    s[10] = z;
    multiply(matrix(), s);
}

void glFrustum(GLdouble left, GLdouble right, GLdouble bottom, GLdouble top, GLdouble zNear, GLdouble zFar)
{
    GLfloat f[16];

    changing();
    if (right == left || top == bottom || zFar == zNear)
        return;
    memset(f, 0, sizeof f);
    f[0] = (GLfloat) (2 * zNear / (right - left));
    f[5] = (GLfloat) (2 * zNear / (top - bottom));
    f[8] = (GLfloat) ((right + left) / (right - left));
    f[9] = (GLfloat) ((top + bottom) / (top - bottom));
    f[10] = (GLfloat) (-(zFar + zNear) / (zFar - zNear));
    f[11] = -1;
    f[14] = (GLfloat) (-2 * zFar * zNear / (zFar - zNear));
    multiply(matrix(), f);
}

void glOrtho(GLdouble left, GLdouble right, GLdouble bottom, GLdouble top, GLdouble zNear, GLdouble zFar)
{
    GLfloat o[16];

    changing();
    if (right == left || top == bottom || zFar == zNear)
        return;
    identity(o);
    o[0] = (GLfloat) (2 / (right - left));
    o[5] = (GLfloat) (2 / (top - bottom));
    o[10] = (GLfloat) (-2 / (zFar - zNear));
    o[12] = (GLfloat) (-(right + left) / (right - left));
    o[13] = (GLfloat) (-(top + bottom) / (top - bottom));
    o[14] = (GLfloat) (-(zFar + zNear) / (zFar - zNear));
    multiply(matrix(), o);
}

/* ---- GLU: setting up a view --------------------------------------------------------------- */

void gluPerspective(GLdouble fovy, GLdouble aspect, GLdouble zNear, GLdouble zFar)
{
    GLdouble top = zNear * tan(fovy * 3.14159265358979323846 / 360.0);

    glFrustum(-top * aspect, top * aspect, -top, top, zNear, zFar);
}

void gluOrtho2D(GLdouble left, GLdouble right, GLdouble bottom, GLdouble top)
{
    glOrtho(left, right, bottom, top, -1, 1);
}

void gluLookAt(GLdouble eyeX, GLdouble eyeY, GLdouble eyeZ, GLdouble centerX, GLdouble centerY, GLdouble centerZ, GLdouble upX,
               GLdouble upY, GLdouble upZ)
{
    GLdouble f[3] = {centerX - eyeX, centerY - eyeY, centerZ - eyeZ}, s[3], u[3], length;
    GLfloat m[16];

    length = sqrt(f[0] * f[0] + f[1] * f[1] + f[2] * f[2]);
    if (length == 0)
        return;
    f[0] /= length, f[1] /= length, f[2] /= length;
    /* s = f across up, then u = s across f */
    s[0] = f[1] * upZ - f[2] * upY;
    s[1] = f[2] * upX - f[0] * upZ;
    s[2] = f[0] * upY - f[1] * upX;
    length = sqrt(s[0] * s[0] + s[1] * s[1] + s[2] * s[2]);
    if (length == 0)
        return;
    s[0] /= length, s[1] /= length, s[2] /= length;
    u[0] = s[1] * f[2] - s[2] * f[1];
    u[1] = s[2] * f[0] - s[0] * f[2];
    u[2] = s[0] * f[1] - s[1] * f[0];
    identity(m);
    m[0] = (GLfloat) s[0], m[4] = (GLfloat) s[1], m[8] = (GLfloat) s[2];
    m[1] = (GLfloat) u[0], m[5] = (GLfloat) u[1], m[9] = (GLfloat) u[2];
    m[2] = (GLfloat) -f[0], m[6] = (GLfloat) -f[1], m[10] = (GLfloat) -f[2];
    glMultMatrixf(m);
    glTranslatef((GLfloat) -eyeX, (GLfloat) -eyeY, (GLfloat) -eyeZ);
}

/* ---- the fixed recipe --------------------------------------------------------------------- */

void glTexEnvf(GLenum target, GLenum pname, GLfloat param)
{
    glTexEnvi(target, pname, (GLint) param);
}

void glTexEnvi(GLenum target, GLenum pname, GLint param)
{
    if (target != GL_TEXTURE_ENV || pname != GL_TEXTURE_ENV_MODE || (GLenum) param == env_mode)
        return;
    flush();
    env_mode = (GLenum) param;
}

void glTexEnvfv(GLenum target, GLenum pname, const GLfloat *params)
{
    if (target != GL_TEXTURE_ENV)
        return;
    if (pname == GL_TEXTURE_ENV_MODE) {
        glTexEnvi(target, pname, (GLint) params[0]);
    } else if (pname == GL_TEXTURE_ENV_COLOR && memcmp(env_colour, params, sizeof env_colour)) {
        flush();
        memcpy(env_colour, params, sizeof env_colour);
    }
}

void glAlphaFunc(GLenum func, GLclampf ref)
{
    if (func < GL_NEVER || func > GL_ALWAYS)
        return;
    ref = ref < 0 ? 0 : ref > 1 ? 1 : ref;
    if (func == alpha_func && ref == alpha_reference)
        return;
    flush();
    alpha_func = func;
    alpha_reference = ref;
}

/* Whether a capability is on: those OpenGL 1 keeps here, and those OpenGL ES 3 has. */
static int *capability(GLenum cap)
{
    switch (cap) {
    case GL_TEXTURE_2D: return &texturing;
    case GL_ALPHA_TEST: return &alpha_testing;
    case GL_BLEND: return &es.blend;
    case GL_DEPTH_TEST: return &es.depth_test;
    case GL_CULL_FACE: return &es.cull_face;
    case GL_SCISSOR_TEST: return &es.scissor_test;
    case GL_POLYGON_OFFSET_FILL: return &es.polygon_offset;
    case GL_STENCIL_TEST: return &es.stencil_test;
    default: return NULL; /* fog, lighting and the rest: not here */
    }
}

static void set_capability(GLenum cap, int on)
{
    int *held_now = capability(cap);

    ONCE();
    if (!held_now || *held_now == on)
        return;
    flush();
    *held_now = on;
    if (cap != GL_TEXTURE_2D && cap != GL_ALPHA_TEST) {
        if (on)
            glEnable(cap);
        else
            glDisable(cap);
    }
}

void gl1_Enable(GLenum cap)
{
    set_capability(cap, 1);
}

void gl1_Disable(GLenum cap)
{
    set_capability(cap, 0);
}

void gl1_DepthFunc(GLenum func)
{
    ONCE();
    if (func == es.depth_func)
        return;
    flush();
    glDepthFunc(es.depth_func = func);
}

void gl1_DepthMask(GLboolean flag)
{
    ONCE();
    flag = flag ? GL_TRUE : GL_FALSE;
    if (flag == es.depth_mask)
        return;
    flush();
    glDepthMask(es.depth_mask = flag);
}

void gl1_CullFace(GLenum mode)
{
    ONCE();
    if (mode == es.cull_mode)
        return;
    flush();
    glCullFace(es.cull_mode = mode);
}

void gl1_FrontFace(GLenum mode)
{
    ONCE();
    if (mode == es.front_face)
        return;
    flush();
    glFrontFace(es.front_face = mode);
}

void gl1_BlendFunc(GLenum sfactor, GLenum dfactor)
{
    ONCE();
    if (sfactor == es.blend_from && dfactor == es.blend_to)
        return;
    flush();
    glBlendFunc(es.blend_from = sfactor, es.blend_to = dfactor);
}

void gl1_ColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha)
{
    GLboolean mask[4] = {red ? GL_TRUE : GL_FALSE, green ? GL_TRUE : GL_FALSE, blue ? GL_TRUE : GL_FALSE, alpha ? GL_TRUE : GL_FALSE};

    ONCE();
    if (!memcmp(mask, es.mask, sizeof mask))
        return;
    flush();
    memcpy(es.mask, mask, sizeof mask);
    glColorMask(mask[0], mask[1], mask[2], mask[3]);
}

void gl1_PolygonOffset(GLfloat factor, GLfloat units)
{
    ONCE();
    if (factor == es.offset_factor && units == es.offset_units)
        return;
    flush();
    glPolygonOffset(es.offset_factor = factor, es.offset_units = units);
}

void gl1_LineWidth(GLfloat width)
{
    ONCE();
    if (width == es.line_width)
        return;
    flush();
    glLineWidth(es.line_width = width);
}

void glDepthRange(GLclampd zNear, GLclampd zFar)
{
    GLfloat n = (GLfloat) zNear, f = (GLfloat) zFar;

    ONCE();
    if (n == es.depth_near && f == es.depth_far)
        return;
    flush();
    glDepthRangef(es.depth_near = n, es.depth_far = f);
}

void glClearDepth(GLclampd depth)
{
    ONCE();
    glClearDepthf((GLfloat) depth);
}

void glShadeModel(GLenum mode)
{
    (void) mode; /* colours are always blended across a shape */
}

void glPolygonMode(GLenum face, GLenum mode)
{
    (void) face;
    (void) mode; /* always filled */
}

void glDrawBuffer(GLenum mode)
{
    (void) mode; /* a page's canvas has the one picture */
}

void gl1_ReadBuffer(GLenum mode)
{
    (void) mode;
}

void glFogf(GLenum pname, GLfloat param)
{
    (void) pname;
    (void) param;
}

void glFogi(GLenum pname, GLint param)
{
    (void) pname;
    (void) param;
}

void glFogfv(GLenum pname, const GLfloat *params)
{
    (void) pname;
    (void) params;
}

void gl1_Hint(GLenum target, GLenum mode)
{
    (void) target;
    (void) mode;
}

/* ---- the picture as a whole --------------------------------------------------------------- */

void gl1_Viewport(GLint x, GLint y, GLsizei width, GLsizei height)
{
    ONCE();
    flush();
    glViewport(x, y, width, height);
}

void gl1_Scissor(GLint x, GLint y, GLsizei width, GLsizei height)
{
    ONCE();
    flush();
    glScissor(x, y, width, height);
}

void gl1_Clear(GLbitfield mask)
{
    ONCE();
    flush();
    glClear(mask);
}

void gl1_ClearColor(GLclampf red, GLclampf green, GLclampf blue, GLclampf alpha)
{
    ONCE();
    glClearColor(red, green, blue, alpha);
}

void gl1_Finish(void)
{
    ONCE();
    flush();
    glFinish();
}

void gl1_Flush(void)
{
    ONCE();
    flush();
    glFlush();
}

GLenum gl1_GetError(void)
{
    ONCE();
    return glGetError();
}

const GLubyte *gl1_GetString(GLenum name)
{
    switch (name) {
    case GL_VENDOR: return (const GLubyte *) "GridCraft";
    case GL_RENDERER: return (const GLubyte *) "OpenGL 1 on the C Development Kit's OpenGL ES 3";
    case GL_VERSION: return (const GLubyte *) "1.1";
    case GL_EXTENSIONS: return (const GLubyte *) "";
    default: return NULL;
    }
}

void gl1_GetFloatv(GLenum pname, GLfloat *params)
{
    GLint values[4] = {0, 0, 0, 0};
    int count = pname == GL_VIEWPORT ? 4 : 1, i;

    ONCE();
    if (pname == GL_MODELVIEW_MATRIX) {
        memcpy(params, modelview[modelview_top], sizeof modelview[0]);
        return;
    }
    if (pname == GL_PROJECTION_MATRIX) {
        memcpy(params, projection[projection_top], sizeof projection[0]);
        return;
    }
    glGetIntegerv(pname, values);
    for (i = 0; i < count; i++)
        params[i] = (GLfloat) values[i];
}

void gl1_GetIntegerv(GLenum pname, GLint *params)
{
    ONCE();
    glGetIntegerv(pname, params);
}

void gl1_PixelStorei(GLenum pname, GLint param)
{
    ONCE();
    if (param != 1 && param != 2 && param != 4 && param != 8)
        return;
    if (pname == GL_UNPACK_ALIGNMENT)
        unpack_alignment = param;
    else if (pname == GL_PACK_ALIGNMENT)
        pack_alignment = param;
    else
        return;
    glPixelStorei(pname, param);
}

static int bytes_a_dot(GLenum format)
{
    switch (format) {
    case GL_RGBA: return 4;
    case GL_RGB: return 3;
    case GL_LUMINANCE_ALPHA: return 2;
    case GL_LUMINANCE:
    case GL_ALPHA: return 1;
    default: return 0;
    }
}

static size_t row_bytes(GLsizei width, GLenum format, GLint alignment)
{
    size_t row = (size_t) width * (size_t) bytes_a_dot(format);
    return (row + (size_t) alignment - 1) / (size_t) alignment * (size_t) alignment;
}

void gl1_ReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, GLvoid *pixels)
{
    GLubyte *read, *to = pixels;
    size_t step;
    GLsizei row, dot;

    ONCE();
    flush();
    if (type != GL_UNSIGNED_BYTE || width <= 0 || height <= 0 || !pixels)
        return;
    if (format == GL_RGBA) {
        glReadPixels(x, y, width, height, GL_RGBA, GL_UNSIGNED_BYTE, pixels);
        return;
    }
    if (format != GL_RGB && format != GL_LUMINANCE && format != GL_ALPHA)
        return;
    /* OpenGL ES 3 reads red, green, blue and alpha: the rest are made from that. */
    read = malloc((size_t) width * (size_t) height * 4);
    if (!read)
        return;
    glPixelStorei(GL_PACK_ALIGNMENT, 1);
    glReadPixels(x, y, width, height, GL_RGBA, GL_UNSIGNED_BYTE, read);
    glPixelStorei(GL_PACK_ALIGNMENT, pack_alignment);
    step = row_bytes(width, format, pack_alignment);
    for (row = 0; row < height; row++) {
        GLubyte *out = to + (size_t) row * step;
        const GLubyte *in = read + (size_t) row * (size_t) width * 4;
        for (dot = 0; dot < width; dot++, in += 4) {
            if (format == GL_RGB) {
                *out++ = in[0];
                *out++ = in[1];
                *out++ = in[2];
            } else {
                *out++ = format == GL_ALPHA ? in[3] : in[0];
            }
        }
    }
    free(read);
}

/* ---- textures ----------------------------------------------------------------------------- */

void gl1_GenTextures(GLsizei n, GLuint *textures)
{
    GLuint name = 1;
    GLsizei i;

    for (i = 0; i < n; i++) {
        while (name < NAMES && (names[name].given || names[name].real))
            name++;
        if (name >= NAMES) {
            textures[i] = 0;
            continue;
        }
        names[name].given = 1;
        textures[i] = name;
    }
}

void gl1_DeleteTextures(GLsizei n, const GLuint *textures)
{
    GLsizei i;

    ONCE();
    flush();
    for (i = 0; i < n; i++) {
        GLuint name = textures[i];
        if (!name || name >= NAMES)
            continue;
        if (names[name].real)
            glDeleteTextures(1, &names[name].real);
        memset(&names[name], 0, sizeof names[name]);
        if (bound == name) {
            bound = 0;
            glBindTexture(GL_TEXTURE_2D, real_texture(0));
        }
    }
}

void gl1_BindTexture(GLenum target, GLuint texture)
{
    ONCE();
    if (target != GL_TEXTURE_2D || texture >= NAMES || (texture == bound && names[texture].real))
        return;
    flush();
    bound = texture;
    glBindTexture(GL_TEXTURE_2D, real_texture(texture));
}

/*
 * How a texture is held, by the kind OpenGL 1 was told (`internal`) and what it was handed
 * (`format`). OpenGL ES 3 holds a texture in the format it is handed; kinds it has not (alpha
 * alone, intensity) are held in the red of a luminance texture, and the shader is told.
 */
static void decide(GLint internal, GLenum format, GLenum *uploaded, GLubyte *takes)
{
    switch (internal) {
    case 1:
    case GL_LUMINANCE:
    case GL_LUMINANCE8:
    case 3:
    case GL_RGB:
    case GL_RGB5:
    case GL_RGB8:
        *uploaded = format == GL_RGB || format == GL_RGBA || format == GL_LUMINANCE ? format : GL_RGB;
        *takes = TAKES_COLOUR;
        break;
    case GL_ALPHA:
    case GL_ALPHA8:
        *uploaded = GL_LUMINANCE;
        *takes = TAKES_ALPHA;
        break;
    case GL_INTENSITY:
    case GL_INTENSITY8:
        *uploaded = GL_LUMINANCE;
        *takes = TAKES_ONE;
        break;
    default: /* 2, 4, luminance and alpha, RGBA and the rest */
        *uploaded = format == GL_RGB ? GL_RGB : GL_RGBA;
        *takes = TAKES_BOTH;
        break;
    }
}

/*
 * The dots handed over, as the format the texture is held in: `pixels` as it is when it is that
 * already, else a copy made here (packed tight) that the caller frees. NULL when it can't be.
 */
static const GLubyte *as_held(const GLubyte *pixels, GLsizei width, GLsizei height, GLenum format, GLenum uploaded, GLubyte takes,
                              GLubyte **made)
{
    size_t step = row_bytes(width, format, unpack_alignment);
    int each = bytes_a_dot(format), to_each = bytes_a_dot(uploaded);
    GLubyte *out;
    GLsizei row, dot;

    *made = NULL;
    if (!each || !to_each)
        return NULL;
    if (format == uploaded && takes != TAKES_ALPHA)
        return pixels;
    out = malloc((size_t) width * (size_t) height * (size_t) to_each);
    if (!out)
        return NULL;
    *made = out;
    for (row = 0; row < height; row++) {
        const GLubyte *in = pixels + (size_t) row * step;
        for (dot = 0; dot < width; dot++, in += each) {
            GLubyte r, g, b, a;
            switch (format) {
            case GL_RGBA: r = in[0], g = in[1], b = in[2], a = in[3]; break;
            case GL_RGB: r = in[0], g = in[1], b = in[2], a = 255; break;
            case GL_LUMINANCE_ALPHA: r = g = b = in[0], a = in[1]; break;
            case GL_ALPHA: r = g = b = 0, a = in[0]; break;
            default: r = g = b = in[0], a = 255; break;
            }
            if (uploaded == GL_LUMINANCE) {
                *out++ = format == GL_ALPHA || format == GL_LUMINANCE ? in[0] : takes == TAKES_ALPHA ? a : r;
            } else {
                *out++ = r;
                *out++ = g;
                *out++ = b;
                if (uploaded == GL_RGBA)
                    *out++ = a;
            }
        }
    }
    return *made;
}

void gl1_TexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format,
                    GLenum type, const GLvoid *pixels)
{
    GLubyte *made = NULL;
    const GLubyte *dots = NULL;

    ONCE();
    if (target != GL_TEXTURE_2D || type != GL_UNSIGNED_BYTE || border || width < 0 || height < 0 || !bytes_a_dot(format))
        return;
    flush();
    real_texture(bound);
    if (level == 0)
        decide(internalformat, format, &names[bound].uploaded, &names[bound].takes);
    if (pixels) {
        dots = as_held(pixels, width, height, format, names[bound].uploaded, names[bound].takes, &made);
        if (!dots)
            return;
    }
    if (made)
        glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
    glTexImage2D(GL_TEXTURE_2D, level, (GLint) names[bound].uploaded, width, height, 0, names[bound].uploaded, GL_UNSIGNED_BYTE, dots);
    if (made) {
        glPixelStorei(GL_UNPACK_ALIGNMENT, unpack_alignment);
        free(made);
    }
}

void gl1_TexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format,
                       GLenum type, const GLvoid *pixels)
{
    GLubyte *made = NULL;
    const GLubyte *dots;

    ONCE();
    if (target != GL_TEXTURE_2D || type != GL_UNSIGNED_BYTE || !pixels || width <= 0 || height <= 0 || !bytes_a_dot(format))
        return;
    flush();
    real_texture(bound);
    dots = as_held(pixels, width, height, format, names[bound].uploaded, names[bound].takes, &made);
    if (!dots)
        return;
    if (made)
        glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
    glTexSubImage2D(GL_TEXTURE_2D, level, xoffset, yoffset, width, height, names[bound].uploaded, GL_UNSIGNED_BYTE, dots);
    if (made) {
        glPixelStorei(GL_UNPACK_ALIGNMENT, unpack_alignment);
        free(made);
    }
}

void gl1_TexParameteri(GLenum target, GLenum pname, GLint param)
{
    ONCE();
    if (target != GL_TEXTURE_2D)
        return;
    flush();
    real_texture(bound);
    /* OpenGL 1's clamp, which OpenGL ES 3 has not, is near enough its clamp to the edge. */
    if ((pname == GL_TEXTURE_WRAP_S || pname == GL_TEXTURE_WRAP_T) && param == GL_CLAMP)
        param = GL_CLAMP_TO_EDGE;
    glTexParameteri(GL_TEXTURE_2D, pname, param);
}

void gl1_TexParameterf(GLenum target, GLenum pname, GLfloat param)
{
    gl1_TexParameteri(target, pname, (GLint) param);
}
"##;

#[cfg(test)]
mod tests {
    use super::*;

    /// The calls a header declares: the name before `(` on each line that begins a declaration.
    fn declared(header: &str) -> Vec<&str> {
        header
            .lines()
            .filter(|line| !line.starts_with([' ', '#', '/', '*', 't']) && line.contains('('))
            .filter_map(|line| line.split('(').next()?.rsplit([' ', '*']).next())
            .filter(|name| name.starts_with("gl"))
            .collect()
    }

    /// Whether `source` defines `call`: a line of its own (not indented, not a declaration) that names it.
    fn defines(source: &str, call: &str) -> bool {
        source.lines().any(|line| {
            !line.starts_with(' ') && !line.ends_with(';') && (line.contains(&format!(" {call}(")) || line.contains(&format!("*{call}(")))
        })
    }

    #[test]
    fn every_call_the_headers_declare_is_one_the_kits_opengl_1_defines() {
        let calls: Vec<&str> = declared(HEADER).into_iter().chain(declared(GLU_HEADER)).collect();
        for call in
            ["glBegin", "glVertex3f", "glMatrixMode", "glTexEnvf", "gl1_Enable", "gl1_TexImage2D", "gl1_GetString", "gluPerspective", "gluLookAt"]
        {
            assert!(calls.contains(&call), "{call} is declared");
        }
        assert!(calls.len() >= 70, "{}", calls.len());
        for call in &calls {
            assert!(defines(SOURCE, call), "{call} is declared and not defined");
        }
    }

    #[test]
    fn what_opengl_1_shares_with_opengl_es_3_is_renamed_and_answered_under_the_new_name() {
        let es: Vec<&str> = crate::c::GL_HEADER.lines().filter_map(|line| line.strip_prefix("GRIDCRAFT_GL(")?.split(')').next()).collect();
        let renamed: Vec<(&str, &str)> = HEADER
            .lines()
            .filter_map(|line| {
                let mut words = line.strip_prefix("#define gl")?.split(' ');
                Some((line.split(' ').nth(1)?, words.nth(1)?))
            })
            .collect();
        assert!(renamed.len() >= 30, "{}", renamed.len());
        for (name, to) in &renamed {
            assert_eq!(*to, format!("gl1_{}", &name[2..]), "{name}");
            assert!(defines(SOURCE, to), "{to}");
        }
        // Every call of OpenGL ES 3's that OpenGL 1 has as well is renamed: none is left to be
        // asked of the page by a program that meant OpenGL 1's.
        for call in ["glEnable", "glBindTexture", "glTexImage2D", "glViewport", "glClear", "glBlendFunc", "glReadPixels", "glDepthMask"] {
            assert!(es.contains(&call) && renamed.iter().any(|(name, _)| name == &call), "{call}");
        }
        // The kit's OpenGL 1 is inside: it reads the header without the new names, and calls OpenGL ES 3.
        assert!(SOURCE.contains("#define GRIDCRAFT_GL1_INSIDE\n#include <GL/gl.h>") && HEADER.contains("#ifndef GRIDCRAFT_GL1_INSIDE"));
        assert!(HEADER.contains("#include <GLES3/gl3.h>") && GLU_HEADER.contains("#include <GL/gl.h>"));
    }

    #[test]
    fn it_is_plain_text_of_the_kits_own() {
        for text in [HEADER, GLU_HEADER, SOURCE] {
            assert!(text.is_ascii() && !text.contains('\r'));
            assert!(text.starts_with("/*\n * GridCraft's C Development Kit:"), "{}", &text[..60]);
            assert!(!text.contains("Quake"), "it is the kit's, not a game's");
        }
    }
}
