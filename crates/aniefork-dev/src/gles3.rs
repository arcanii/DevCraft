//! OpenGL ES 3.0 for the C Development Kit's programs: the header `<GLES3/gl3.h>` ([`HEADER`]),
//! and the kit's own C beside it ([`SOURCE`]).
//!
//! The owner asked for it (2026-10-19: "Let's do the graphics work first : Fuller OpenGL ES").
//! Until then the header held the part of OpenGL ES 3 the first programs drew with (78 calls).
//! Now it holds the whole of OpenGL ES 3.0: every call, answered by the loader with the canvas's
//! WebGL 2 ([`crate::c_program`]), and every number, read from WebGL 2 itself in the web view a
//! docked panel is shown in, with the few OpenGL ES 3.0 has that WebGL does not. What WebGL
//! can't do is said in the header, and answered with OpenGL's own errors.
//!
//! Five calls hand back a pointer into the program's memory (`glGetString`, `glGetStringi`,
//! `glMapBufferRange`, `glUnmapBuffer`, `glFlushMappedBufferRange`), and the loader has none of
//! the program's memory to hand: they are done in the program, by the kit's C ([`SOURCE`]),
//! which is built into every program that includes the header ([`crate::c`]). It asks the
//! loader for what it needs under names of the kit's own (`gridcraft_gl...`).
//!
//! Text inside Rust source, as the loader is: the repository holds no C file.

/// `<GLES3/gl3.h>`, GridCraft's own, supplied to every build.
pub const HEADER: &str = r#"/*
 * GridCraft's C Development Kit: OpenGL ES 3.0, as the kit's loader hands it to a program.
 *
 * A C project's program is WebAssembly, run by its panel's page. When the page names a canvas
 * to draw on (CProgram.load({ canvas })), the loader answers the calls below with that canvas's
 * WebGL 2, which is OpenGL ES 3.0 as a page has it. A call made with no canvas does nothing.
 *
 * Every call of OpenGL ES 3.0 is here, and every number. What a page can't do is said where it
 * is declared, and is not done: there are no binaries of programs or shaders (glShaderBinary,
 * glProgramBinary say GL_INVALID_ENUM), vertex data and indices come from buffers and not from
 * the program's memory, a fence is waited for no longer than the page allows, and the answer
 * to a query comes a while after it is asked. The extensions the loader turns on, where the
 * canvas has them, are named by glGetString(GL_EXTENSIONS); their numbers are at the end.
 *
 * The names and numbers are OpenGL's own, so that code written for OpenGL ES reads the same; the
 * numbers were read from WebGL 2 itself. The file is GridCraft's, written for this kit, and is
 * not a project's to change: GridCraft supplies it when it builds.
 */
#ifndef GRIDCRAFT_GLES3_GL3_H
#define GRIDCRAFT_GLES3_GL3_H

#ifdef __cplusplus
extern "C" {
#endif

typedef void GLvoid;
typedef char GLchar;
typedef unsigned char GLboolean;
typedef signed char GLbyte;
typedef unsigned char GLubyte;
typedef short GLshort;
typedef unsigned short GLushort;
typedef unsigned short GLhalf;
typedef int GLint;
typedef int GLsizei;
typedef int GLfixed;
typedef unsigned int GLuint;
typedef unsigned int GLenum;
typedef unsigned int GLbitfield;
typedef float GLfloat;
typedef float GLclampf;
typedef long GLintptr;
typedef long GLsizeiptr;
typedef long long GLint64;
typedef unsigned long long GLuint64;
typedef struct __GLsync *GLsync;

#define GL_ES_VERSION_2_0 1
#define GL_ES_VERSION_3_0 1

/* The numbers OpenGL ES 3.0 names things by. */
#define GL_DEPTH_BUFFER_BIT 0x0100
#define GL_STENCIL_BUFFER_BIT 0x0400
#define GL_COLOR_BUFFER_BIT 0x4000
#define GL_POINTS 0x0000
#define GL_LINES 0x0001
#define GL_LINE_LOOP 0x0002
#define GL_LINE_STRIP 0x0003
#define GL_TRIANGLES 0x0004
#define GL_TRIANGLE_STRIP 0x0005
#define GL_TRIANGLE_FAN 0x0006
#define GL_ZERO 0x0000
#define GL_ONE 0x0001
#define GL_SRC_COLOR 0x0300
#define GL_ONE_MINUS_SRC_COLOR 0x0301
#define GL_SRC_ALPHA 0x0302
#define GL_ONE_MINUS_SRC_ALPHA 0x0303
#define GL_DST_ALPHA 0x0304
#define GL_ONE_MINUS_DST_ALPHA 0x0305
#define GL_DST_COLOR 0x0306
#define GL_ONE_MINUS_DST_COLOR 0x0307
#define GL_SRC_ALPHA_SATURATE 0x0308
#define GL_FUNC_ADD 0x8006
#define GL_BLEND_EQUATION 0x8009
#define GL_BLEND_EQUATION_RGB 0x8009
#define GL_BLEND_EQUATION_ALPHA 0x883D
#define GL_FUNC_SUBTRACT 0x800A
#define GL_FUNC_REVERSE_SUBTRACT 0x800B
#define GL_BLEND_DST_RGB 0x80C8
#define GL_BLEND_SRC_RGB 0x80C9
#define GL_BLEND_DST_ALPHA 0x80CA
#define GL_BLEND_SRC_ALPHA 0x80CB
#define GL_CONSTANT_COLOR 0x8001
#define GL_ONE_MINUS_CONSTANT_COLOR 0x8002
#define GL_CONSTANT_ALPHA 0x8003
#define GL_ONE_MINUS_CONSTANT_ALPHA 0x8004
#define GL_BLEND_COLOR 0x8005
#define GL_ARRAY_BUFFER 0x8892
#define GL_ELEMENT_ARRAY_BUFFER 0x8893
#define GL_ARRAY_BUFFER_BINDING 0x8894
#define GL_ELEMENT_ARRAY_BUFFER_BINDING 0x8895
#define GL_STREAM_DRAW 0x88E0
#define GL_STATIC_DRAW 0x88E4
#define GL_DYNAMIC_DRAW 0x88E8
#define GL_BUFFER_SIZE 0x8764
#define GL_BUFFER_USAGE 0x8765
#define GL_CURRENT_VERTEX_ATTRIB 0x8626
#define GL_FRONT 0x0404
#define GL_BACK 0x0405
#define GL_FRONT_AND_BACK 0x0408
#define GL_TEXTURE_2D 0x0DE1
#define GL_CULL_FACE 0x0B44
#define GL_BLEND 0x0BE2
#define GL_DITHER 0x0BD0
#define GL_STENCIL_TEST 0x0B90
#define GL_DEPTH_TEST 0x0B71
#define GL_SCISSOR_TEST 0x0C11
#define GL_POLYGON_OFFSET_FILL 0x8037
#define GL_SAMPLE_ALPHA_TO_COVERAGE 0x809E
#define GL_SAMPLE_COVERAGE 0x80A0
#define GL_NO_ERROR 0x0000
#define GL_INVALID_ENUM 0x0500
#define GL_INVALID_VALUE 0x0501
#define GL_INVALID_OPERATION 0x0502
#define GL_OUT_OF_MEMORY 0x0505
#define GL_CW 0x0900
#define GL_CCW 0x0901
#define GL_LINE_WIDTH 0x0B21
#define GL_ALIASED_POINT_SIZE_RANGE 0x846D
#define GL_ALIASED_LINE_WIDTH_RANGE 0x846E
#define GL_CULL_FACE_MODE 0x0B45
#define GL_FRONT_FACE 0x0B46
#define GL_DEPTH_RANGE 0x0B70
#define GL_DEPTH_WRITEMASK 0x0B72
#define GL_DEPTH_CLEAR_VALUE 0x0B73
#define GL_DEPTH_FUNC 0x0B74
#define GL_STENCIL_CLEAR_VALUE 0x0B91
#define GL_STENCIL_FUNC 0x0B92
#define GL_STENCIL_FAIL 0x0B94
#define GL_STENCIL_PASS_DEPTH_FAIL 0x0B95
#define GL_STENCIL_PASS_DEPTH_PASS 0x0B96
#define GL_STENCIL_REF 0x0B97
#define GL_STENCIL_VALUE_MASK 0x0B93
#define GL_STENCIL_WRITEMASK 0x0B98
#define GL_STENCIL_BACK_FUNC 0x8800
#define GL_STENCIL_BACK_FAIL 0x8801
#define GL_STENCIL_BACK_PASS_DEPTH_FAIL 0x8802
#define GL_STENCIL_BACK_PASS_DEPTH_PASS 0x8803
#define GL_STENCIL_BACK_REF 0x8CA3
#define GL_STENCIL_BACK_VALUE_MASK 0x8CA4
#define GL_STENCIL_BACK_WRITEMASK 0x8CA5
#define GL_VIEWPORT 0x0BA2
#define GL_SCISSOR_BOX 0x0C10
#define GL_COLOR_CLEAR_VALUE 0x0C22
#define GL_COLOR_WRITEMASK 0x0C23
#define GL_UNPACK_ALIGNMENT 0x0CF5
#define GL_PACK_ALIGNMENT 0x0D05
#define GL_MAX_TEXTURE_SIZE 0x0D33
#define GL_MAX_VIEWPORT_DIMS 0x0D3A
#define GL_SUBPIXEL_BITS 0x0D50
#define GL_RED_BITS 0x0D52
#define GL_GREEN_BITS 0x0D53
#define GL_BLUE_BITS 0x0D54
#define GL_ALPHA_BITS 0x0D55
#define GL_DEPTH_BITS 0x0D56
#define GL_STENCIL_BITS 0x0D57
#define GL_POLYGON_OFFSET_UNITS 0x2A00
#define GL_POLYGON_OFFSET_FACTOR 0x8038
#define GL_TEXTURE_BINDING_2D 0x8069
#define GL_SAMPLE_BUFFERS 0x80A8
#define GL_SAMPLES 0x80A9
#define GL_SAMPLE_COVERAGE_VALUE 0x80AA
#define GL_SAMPLE_COVERAGE_INVERT 0x80AB
#define GL_COMPRESSED_TEXTURE_FORMATS 0x86A3
#define GL_DONT_CARE 0x1100
#define GL_FASTEST 0x1101
#define GL_NICEST 0x1102
#define GL_GENERATE_MIPMAP_HINT 0x8192
#define GL_BYTE 0x1400
#define GL_UNSIGNED_BYTE 0x1401
#define GL_SHORT 0x1402
#define GL_UNSIGNED_SHORT 0x1403
#define GL_INT 0x1404
#define GL_UNSIGNED_INT 0x1405
#define GL_FLOAT 0x1406
#define GL_DEPTH_COMPONENT 0x1902
#define GL_ALPHA 0x1906
#define GL_RGB 0x1907
#define GL_RGBA 0x1908
#define GL_LUMINANCE 0x1909
#define GL_LUMINANCE_ALPHA 0x190A
#define GL_UNSIGNED_SHORT_4_4_4_4 0x8033
#define GL_UNSIGNED_SHORT_5_5_5_1 0x8034
#define GL_UNSIGNED_SHORT_5_6_5 0x8363
#define GL_FRAGMENT_SHADER 0x8B30
#define GL_VERTEX_SHADER 0x8B31
#define GL_MAX_VERTEX_ATTRIBS 0x8869
#define GL_MAX_VERTEX_UNIFORM_VECTORS 0x8DFB
#define GL_MAX_VARYING_VECTORS 0x8DFC
#define GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS 0x8B4D
#define GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS 0x8B4C
#define GL_MAX_TEXTURE_IMAGE_UNITS 0x8872
#define GL_MAX_FRAGMENT_UNIFORM_VECTORS 0x8DFD
#define GL_SHADER_TYPE 0x8B4F
#define GL_DELETE_STATUS 0x8B80
#define GL_LINK_STATUS 0x8B82
#define GL_VALIDATE_STATUS 0x8B83
#define GL_ATTACHED_SHADERS 0x8B85
#define GL_ACTIVE_UNIFORMS 0x8B86
#define GL_ACTIVE_ATTRIBUTES 0x8B89
#define GL_SHADING_LANGUAGE_VERSION 0x8B8C
#define GL_CURRENT_PROGRAM 0x8B8D
#define GL_NEVER 0x0200
#define GL_LESS 0x0201
#define GL_EQUAL 0x0202
#define GL_LEQUAL 0x0203
#define GL_GREATER 0x0204
#define GL_NOTEQUAL 0x0205
#define GL_GEQUAL 0x0206
#define GL_ALWAYS 0x0207
#define GL_KEEP 0x1E00
#define GL_REPLACE 0x1E01
#define GL_INCR 0x1E02
#define GL_DECR 0x1E03
#define GL_INVERT 0x150A
#define GL_INCR_WRAP 0x8507
#define GL_DECR_WRAP 0x8508
#define GL_VENDOR 0x1F00
#define GL_RENDERER 0x1F01
#define GL_VERSION 0x1F02
#define GL_NEAREST 0x2600
#define GL_LINEAR 0x2601
#define GL_NEAREST_MIPMAP_NEAREST 0x2700
#define GL_LINEAR_MIPMAP_NEAREST 0x2701
#define GL_NEAREST_MIPMAP_LINEAR 0x2702
#define GL_LINEAR_MIPMAP_LINEAR 0x2703
#define GL_TEXTURE_MAG_FILTER 0x2800
#define GL_TEXTURE_MIN_FILTER 0x2801
#define GL_TEXTURE_WRAP_S 0x2802
#define GL_TEXTURE_WRAP_T 0x2803
#define GL_TEXTURE 0x1702
#define GL_TEXTURE_CUBE_MAP 0x8513
#define GL_TEXTURE_BINDING_CUBE_MAP 0x8514
#define GL_TEXTURE_CUBE_MAP_POSITIVE_X 0x8515
#define GL_TEXTURE_CUBE_MAP_NEGATIVE_X 0x8516
#define GL_TEXTURE_CUBE_MAP_POSITIVE_Y 0x8517
#define GL_TEXTURE_CUBE_MAP_NEGATIVE_Y 0x8518
#define GL_TEXTURE_CUBE_MAP_POSITIVE_Z 0x8519
#define GL_TEXTURE_CUBE_MAP_NEGATIVE_Z 0x851A
#define GL_MAX_CUBE_MAP_TEXTURE_SIZE 0x851C
#define GL_TEXTURE0 0x84C0
#define GL_TEXTURE1 0x84C1
#define GL_TEXTURE2 0x84C2
#define GL_TEXTURE3 0x84C3
#define GL_TEXTURE4 0x84C4
#define GL_TEXTURE5 0x84C5
#define GL_TEXTURE6 0x84C6
#define GL_TEXTURE7 0x84C7
#define GL_TEXTURE8 0x84C8
#define GL_TEXTURE9 0x84C9
#define GL_TEXTURE10 0x84CA
#define GL_TEXTURE11 0x84CB
#define GL_TEXTURE12 0x84CC
#define GL_TEXTURE13 0x84CD
#define GL_TEXTURE14 0x84CE
#define GL_TEXTURE15 0x84CF
#define GL_TEXTURE16 0x84D0
#define GL_TEXTURE17 0x84D1
#define GL_TEXTURE18 0x84D2
#define GL_TEXTURE19 0x84D3
#define GL_TEXTURE20 0x84D4
#define GL_TEXTURE21 0x84D5
#define GL_TEXTURE22 0x84D6
#define GL_TEXTURE23 0x84D7
#define GL_TEXTURE24 0x84D8
#define GL_TEXTURE25 0x84D9
#define GL_TEXTURE26 0x84DA
#define GL_TEXTURE27 0x84DB
#define GL_TEXTURE28 0x84DC
#define GL_TEXTURE29 0x84DD
#define GL_TEXTURE30 0x84DE
#define GL_TEXTURE31 0x84DF
#define GL_ACTIVE_TEXTURE 0x84E0
#define GL_REPEAT 0x2901
#define GL_CLAMP_TO_EDGE 0x812F
#define GL_MIRRORED_REPEAT 0x8370
#define GL_FLOAT_VEC2 0x8B50
#define GL_FLOAT_VEC3 0x8B51
#define GL_FLOAT_VEC4 0x8B52
#define GL_INT_VEC2 0x8B53
#define GL_INT_VEC3 0x8B54
#define GL_INT_VEC4 0x8B55
#define GL_BOOL 0x8B56
#define GL_BOOL_VEC2 0x8B57
#define GL_BOOL_VEC3 0x8B58
#define GL_BOOL_VEC4 0x8B59
#define GL_FLOAT_MAT2 0x8B5A
#define GL_FLOAT_MAT3 0x8B5B
#define GL_FLOAT_MAT4 0x8B5C
#define GL_SAMPLER_2D 0x8B5E
#define GL_SAMPLER_CUBE 0x8B60
#define GL_VERTEX_ATTRIB_ARRAY_ENABLED 0x8622
#define GL_VERTEX_ATTRIB_ARRAY_SIZE 0x8623
#define GL_VERTEX_ATTRIB_ARRAY_STRIDE 0x8624
#define GL_VERTEX_ATTRIB_ARRAY_TYPE 0x8625
#define GL_VERTEX_ATTRIB_ARRAY_NORMALIZED 0x886A
#define GL_VERTEX_ATTRIB_ARRAY_POINTER 0x8645
#define GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING 0x889F
#define GL_IMPLEMENTATION_COLOR_READ_TYPE 0x8B9A
#define GL_IMPLEMENTATION_COLOR_READ_FORMAT 0x8B9B
#define GL_COMPILE_STATUS 0x8B81
#define GL_LOW_FLOAT 0x8DF0
#define GL_MEDIUM_FLOAT 0x8DF1
#define GL_HIGH_FLOAT 0x8DF2
#define GL_LOW_INT 0x8DF3
#define GL_MEDIUM_INT 0x8DF4
#define GL_HIGH_INT 0x8DF5
#define GL_FRAMEBUFFER 0x8D40
#define GL_RENDERBUFFER 0x8D41
#define GL_RGBA4 0x8056
#define GL_RGB5_A1 0x8057
#define GL_RGB565 0x8D62
#define GL_DEPTH_COMPONENT16 0x81A5
#define GL_STENCIL_INDEX8 0x8D48
#define GL_DEPTH_STENCIL 0x84F9
#define GL_RENDERBUFFER_WIDTH 0x8D42
#define GL_RENDERBUFFER_HEIGHT 0x8D43
#define GL_RENDERBUFFER_INTERNAL_FORMAT 0x8D44
#define GL_RENDERBUFFER_RED_SIZE 0x8D50
#define GL_RENDERBUFFER_GREEN_SIZE 0x8D51
#define GL_RENDERBUFFER_BLUE_SIZE 0x8D52
#define GL_RENDERBUFFER_ALPHA_SIZE 0x8D53
#define GL_RENDERBUFFER_DEPTH_SIZE 0x8D54
#define GL_RENDERBUFFER_STENCIL_SIZE 0x8D55
#define GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE 0x8CD0
#define GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME 0x8CD1
#define GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL 0x8CD2
#define GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE 0x8CD3
#define GL_COLOR_ATTACHMENT0 0x8CE0
#define GL_DEPTH_ATTACHMENT 0x8D00
#define GL_STENCIL_ATTACHMENT 0x8D20
#define GL_DEPTH_STENCIL_ATTACHMENT 0x821A
#define GL_NONE 0x0000
#define GL_FRAMEBUFFER_COMPLETE 0x8CD5
#define GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT 0x8CD6
#define GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT 0x8CD7
#define GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS 0x8CD9
#define GL_FRAMEBUFFER_UNSUPPORTED 0x8CDD
#define GL_FRAMEBUFFER_BINDING 0x8CA6
#define GL_RENDERBUFFER_BINDING 0x8CA7
#define GL_MAX_RENDERBUFFER_SIZE 0x84E8
#define GL_INVALID_FRAMEBUFFER_OPERATION 0x0506
#define GL_READ_BUFFER 0x0C02
#define GL_UNPACK_ROW_LENGTH 0x0CF2
#define GL_UNPACK_SKIP_ROWS 0x0CF3
#define GL_UNPACK_SKIP_PIXELS 0x0CF4
#define GL_PACK_ROW_LENGTH 0x0D02
#define GL_PACK_SKIP_ROWS 0x0D03
#define GL_PACK_SKIP_PIXELS 0x0D04
#define GL_COLOR 0x1800
#define GL_DEPTH 0x1801
#define GL_STENCIL 0x1802
#define GL_RED 0x1903
#define GL_RGB8 0x8051
#define GL_RGBA8 0x8058
#define GL_RGB10_A2 0x8059
#define GL_TEXTURE_BINDING_3D 0x806A
#define GL_UNPACK_SKIP_IMAGES 0x806D
#define GL_UNPACK_IMAGE_HEIGHT 0x806E
#define GL_TEXTURE_3D 0x806F
#define GL_TEXTURE_WRAP_R 0x8072
#define GL_MAX_3D_TEXTURE_SIZE 0x8073
#define GL_UNSIGNED_INT_2_10_10_10_REV 0x8368
#define GL_MAX_ELEMENTS_VERTICES 0x80E8
#define GL_MAX_ELEMENTS_INDICES 0x80E9
#define GL_TEXTURE_MIN_LOD 0x813A
#define GL_TEXTURE_MAX_LOD 0x813B
#define GL_TEXTURE_BASE_LEVEL 0x813C
#define GL_TEXTURE_MAX_LEVEL 0x813D
#define GL_MIN 0x8007
#define GL_MAX 0x8008
#define GL_DEPTH_COMPONENT24 0x81A6
#define GL_MAX_TEXTURE_LOD_BIAS 0x84FD
#define GL_TEXTURE_COMPARE_MODE 0x884C
#define GL_TEXTURE_COMPARE_FUNC 0x884D
#define GL_CURRENT_QUERY 0x8865
#define GL_QUERY_RESULT 0x8866
#define GL_QUERY_RESULT_AVAILABLE 0x8867
#define GL_STREAM_READ 0x88E1
#define GL_STREAM_COPY 0x88E2
#define GL_STATIC_READ 0x88E5
#define GL_STATIC_COPY 0x88E6
#define GL_DYNAMIC_READ 0x88E9
#define GL_DYNAMIC_COPY 0x88EA
#define GL_MAX_DRAW_BUFFERS 0x8824
#define GL_DRAW_BUFFER0 0x8825
#define GL_DRAW_BUFFER1 0x8826
#define GL_DRAW_BUFFER2 0x8827
#define GL_DRAW_BUFFER3 0x8828
#define GL_DRAW_BUFFER4 0x8829
#define GL_DRAW_BUFFER5 0x882A
#define GL_DRAW_BUFFER6 0x882B
#define GL_DRAW_BUFFER7 0x882C
#define GL_DRAW_BUFFER8 0x882D
#define GL_DRAW_BUFFER9 0x882E
#define GL_DRAW_BUFFER10 0x882F
#define GL_DRAW_BUFFER11 0x8830
#define GL_DRAW_BUFFER12 0x8831
#define GL_DRAW_BUFFER13 0x8832
#define GL_DRAW_BUFFER14 0x8833
#define GL_DRAW_BUFFER15 0x8834
#define GL_MAX_FRAGMENT_UNIFORM_COMPONENTS 0x8B49
#define GL_MAX_VERTEX_UNIFORM_COMPONENTS 0x8B4A
#define GL_SAMPLER_3D 0x8B5F
#define GL_SAMPLER_2D_SHADOW 0x8B62
#define GL_FRAGMENT_SHADER_DERIVATIVE_HINT 0x8B8B
#define GL_PIXEL_PACK_BUFFER 0x88EB
#define GL_PIXEL_UNPACK_BUFFER 0x88EC
#define GL_PIXEL_PACK_BUFFER_BINDING 0x88ED
#define GL_PIXEL_UNPACK_BUFFER_BINDING 0x88EF
#define GL_SRGB 0x8C40
#define GL_SRGB8 0x8C41
#define GL_SRGB8_ALPHA8 0x8C43
#define GL_COMPARE_REF_TO_TEXTURE 0x884E
#define GL_RGBA32F 0x8814
#define GL_RGB32F 0x8815
#define GL_RGBA16F 0x881A
#define GL_RGB16F 0x881B
#define GL_VERTEX_ATTRIB_ARRAY_INTEGER 0x88FD
#define GL_MAX_ARRAY_TEXTURE_LAYERS 0x88FF
#define GL_MIN_PROGRAM_TEXEL_OFFSET 0x8904
#define GL_MAX_PROGRAM_TEXEL_OFFSET 0x8905
#define GL_MAX_VARYING_COMPONENTS 0x8B4B
#define GL_TEXTURE_2D_ARRAY 0x8C1A
#define GL_TEXTURE_BINDING_2D_ARRAY 0x8C1D
#define GL_R11F_G11F_B10F 0x8C3A
#define GL_UNSIGNED_INT_10F_11F_11F_REV 0x8C3B
#define GL_RGB9_E5 0x8C3D
#define GL_UNSIGNED_INT_5_9_9_9_REV 0x8C3E
#define GL_TRANSFORM_FEEDBACK_BUFFER_MODE 0x8C7F
#define GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS 0x8C80
#define GL_TRANSFORM_FEEDBACK_VARYINGS 0x8C83
#define GL_TRANSFORM_FEEDBACK_BUFFER_START 0x8C84
#define GL_TRANSFORM_FEEDBACK_BUFFER_SIZE 0x8C85
#define GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN 0x8C88
#define GL_RASTERIZER_DISCARD 0x8C89
#define GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS 0x8C8A
#define GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS 0x8C8B
#define GL_INTERLEAVED_ATTRIBS 0x8C8C
#define GL_SEPARATE_ATTRIBS 0x8C8D
#define GL_TRANSFORM_FEEDBACK_BUFFER 0x8C8E
#define GL_TRANSFORM_FEEDBACK_BUFFER_BINDING 0x8C8F
#define GL_RGBA32UI 0x8D70
#define GL_RGB32UI 0x8D71
#define GL_RGBA16UI 0x8D76
#define GL_RGB16UI 0x8D77
#define GL_RGBA8UI 0x8D7C
#define GL_RGB8UI 0x8D7D
#define GL_RGBA32I 0x8D82
#define GL_RGB32I 0x8D83
#define GL_RGBA16I 0x8D88
#define GL_RGB16I 0x8D89
#define GL_RGBA8I 0x8D8E
#define GL_RGB8I 0x8D8F
#define GL_RED_INTEGER 0x8D94
#define GL_RGB_INTEGER 0x8D98
#define GL_RGBA_INTEGER 0x8D99
#define GL_SAMPLER_2D_ARRAY 0x8DC1
#define GL_SAMPLER_2D_ARRAY_SHADOW 0x8DC4
#define GL_SAMPLER_CUBE_SHADOW 0x8DC5
#define GL_UNSIGNED_INT_VEC2 0x8DC6
#define GL_UNSIGNED_INT_VEC3 0x8DC7
#define GL_UNSIGNED_INT_VEC4 0x8DC8
#define GL_INT_SAMPLER_2D 0x8DCA
#define GL_INT_SAMPLER_3D 0x8DCB
#define GL_INT_SAMPLER_CUBE 0x8DCC
#define GL_INT_SAMPLER_2D_ARRAY 0x8DCF
#define GL_UNSIGNED_INT_SAMPLER_2D 0x8DD2
#define GL_UNSIGNED_INT_SAMPLER_3D 0x8DD3
#define GL_UNSIGNED_INT_SAMPLER_CUBE 0x8DD4
#define GL_UNSIGNED_INT_SAMPLER_2D_ARRAY 0x8DD7
#define GL_DEPTH_COMPONENT32F 0x8CAC
#define GL_DEPTH32F_STENCIL8 0x8CAD
#define GL_FLOAT_32_UNSIGNED_INT_24_8_REV 0x8DAD
#define GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING 0x8210
#define GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE 0x8211
#define GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE 0x8212
#define GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE 0x8213
#define GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE 0x8214
#define GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE 0x8215
#define GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE 0x8216
#define GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE 0x8217
#define GL_FRAMEBUFFER_DEFAULT 0x8218
#define GL_UNSIGNED_INT_24_8 0x84FA
#define GL_DEPTH24_STENCIL8 0x88F0
#define GL_UNSIGNED_NORMALIZED 0x8C17
#define GL_DRAW_FRAMEBUFFER_BINDING 0x8CA6
#define GL_READ_FRAMEBUFFER 0x8CA8
#define GL_DRAW_FRAMEBUFFER 0x8CA9
#define GL_READ_FRAMEBUFFER_BINDING 0x8CAA
#define GL_RENDERBUFFER_SAMPLES 0x8CAB
#define GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER 0x8CD4
#define GL_MAX_COLOR_ATTACHMENTS 0x8CDF
#define GL_COLOR_ATTACHMENT1 0x8CE1
#define GL_COLOR_ATTACHMENT2 0x8CE2
#define GL_COLOR_ATTACHMENT3 0x8CE3
#define GL_COLOR_ATTACHMENT4 0x8CE4
#define GL_COLOR_ATTACHMENT5 0x8CE5
#define GL_COLOR_ATTACHMENT6 0x8CE6
#define GL_COLOR_ATTACHMENT7 0x8CE7
#define GL_COLOR_ATTACHMENT8 0x8CE8
#define GL_COLOR_ATTACHMENT9 0x8CE9
#define GL_COLOR_ATTACHMENT10 0x8CEA
#define GL_COLOR_ATTACHMENT11 0x8CEB
#define GL_COLOR_ATTACHMENT12 0x8CEC
#define GL_COLOR_ATTACHMENT13 0x8CED
#define GL_COLOR_ATTACHMENT14 0x8CEE
#define GL_COLOR_ATTACHMENT15 0x8CEF
#define GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE 0x8D56
#define GL_MAX_SAMPLES 0x8D57
#define GL_HALF_FLOAT 0x140B
#define GL_RG 0x8227
#define GL_RG_INTEGER 0x8228
#define GL_R8 0x8229
#define GL_RG8 0x822B
#define GL_R16F 0x822D
#define GL_R32F 0x822E
#define GL_RG16F 0x822F
#define GL_RG32F 0x8230
#define GL_R8I 0x8231
#define GL_R8UI 0x8232
#define GL_R16I 0x8233
#define GL_R16UI 0x8234
#define GL_R32I 0x8235
#define GL_R32UI 0x8236
#define GL_RG8I 0x8237
#define GL_RG8UI 0x8238
#define GL_RG16I 0x8239
#define GL_RG16UI 0x823A
#define GL_RG32I 0x823B
#define GL_RG32UI 0x823C
#define GL_VERTEX_ARRAY_BINDING 0x85B5
#define GL_R8_SNORM 0x8F94
#define GL_RG8_SNORM 0x8F95
#define GL_RGB8_SNORM 0x8F96
#define GL_RGBA8_SNORM 0x8F97
#define GL_SIGNED_NORMALIZED 0x8F9C
#define GL_COPY_READ_BUFFER 0x8F36
#define GL_COPY_WRITE_BUFFER 0x8F37
#define GL_COPY_READ_BUFFER_BINDING 0x8F36
#define GL_COPY_WRITE_BUFFER_BINDING 0x8F37
#define GL_UNIFORM_BUFFER 0x8A11
#define GL_UNIFORM_BUFFER_BINDING 0x8A28
#define GL_UNIFORM_BUFFER_START 0x8A29
#define GL_UNIFORM_BUFFER_SIZE 0x8A2A
#define GL_MAX_VERTEX_UNIFORM_BLOCKS 0x8A2B
#define GL_MAX_FRAGMENT_UNIFORM_BLOCKS 0x8A2D
#define GL_MAX_COMBINED_UNIFORM_BLOCKS 0x8A2E
#define GL_MAX_UNIFORM_BUFFER_BINDINGS 0x8A2F
#define GL_MAX_UNIFORM_BLOCK_SIZE 0x8A30
#define GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS 0x8A31
#define GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS 0x8A33
#define GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT 0x8A34
#define GL_ACTIVE_UNIFORM_BLOCKS 0x8A36
#define GL_UNIFORM_TYPE 0x8A37
#define GL_UNIFORM_SIZE 0x8A38
#define GL_UNIFORM_BLOCK_INDEX 0x8A3A
#define GL_UNIFORM_OFFSET 0x8A3B
#define GL_UNIFORM_ARRAY_STRIDE 0x8A3C
#define GL_UNIFORM_MATRIX_STRIDE 0x8A3D
#define GL_UNIFORM_IS_ROW_MAJOR 0x8A3E
#define GL_UNIFORM_BLOCK_BINDING 0x8A3F
#define GL_UNIFORM_BLOCK_DATA_SIZE 0x8A40
#define GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS 0x8A42
#define GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES 0x8A43
#define GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER 0x8A44
#define GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER 0x8A46
#define GL_MAX_VERTEX_OUTPUT_COMPONENTS 0x9122
#define GL_MAX_FRAGMENT_INPUT_COMPONENTS 0x9125
#define GL_MAX_SERVER_WAIT_TIMEOUT 0x9111
#define GL_OBJECT_TYPE 0x9112
#define GL_SYNC_CONDITION 0x9113
#define GL_SYNC_STATUS 0x9114
#define GL_SYNC_FLAGS 0x9115
#define GL_SYNC_FENCE 0x9116
#define GL_SYNC_GPU_COMMANDS_COMPLETE 0x9117
#define GL_UNSIGNALED 0x9118
#define GL_SIGNALED 0x9119
#define GL_ALREADY_SIGNALED 0x911A
#define GL_TIMEOUT_EXPIRED 0x911B
#define GL_CONDITION_SATISFIED 0x911C
#define GL_WAIT_FAILED 0x911D
#define GL_SYNC_FLUSH_COMMANDS_BIT 0x0001
#define GL_VERTEX_ATTRIB_ARRAY_DIVISOR 0x88FE
#define GL_ANY_SAMPLES_PASSED 0x8C2F
#define GL_ANY_SAMPLES_PASSED_CONSERVATIVE 0x8D6A
#define GL_SAMPLER_BINDING 0x8919
#define GL_RGB10_A2UI 0x906F
#define GL_INT_2_10_10_10_REV 0x8D9F
#define GL_TRANSFORM_FEEDBACK 0x8E22
#define GL_TRANSFORM_FEEDBACK_PAUSED 0x8E23
#define GL_TRANSFORM_FEEDBACK_ACTIVE 0x8E24
#define GL_TRANSFORM_FEEDBACK_BINDING 0x8E25
#define GL_TEXTURE_IMMUTABLE_FORMAT 0x912F
#define GL_MAX_ELEMENT_INDEX 0x8D6B
#define GL_TEXTURE_IMMUTABLE_LEVELS 0x82DF
#define GL_FALSE 0x0000
#define GL_TRUE 0x0001
#define GL_FIXED 0x140C
#define GL_EXTENSIONS 0x1F03
#define GL_INFO_LOG_LENGTH 0x8B84
#define GL_SHADER_SOURCE_LENGTH 0x8B88
#define GL_ACTIVE_UNIFORM_MAX_LENGTH 0x8B87
#define GL_ACTIVE_ATTRIBUTE_MAX_LENGTH 0x8B8A
#define GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH 0x8C76
#define GL_UNIFORM_BLOCK_NAME_LENGTH 0x8A41
#define GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH 0x8A35
#define GL_SHADER_COMPILER 0x8DFA
#define GL_SHADER_BINARY_FORMATS 0x8DF8
#define GL_NUM_SHADER_BINARY_FORMATS 0x8DF9
#define GL_PROGRAM_BINARY_RETRIEVABLE_HINT 0x8257
#define GL_PROGRAM_BINARY_LENGTH 0x8741
#define GL_NUM_PROGRAM_BINARY_FORMATS 0x87FE
#define GL_PROGRAM_BINARY_FORMATS 0x87FF
#define GL_NUM_COMPRESSED_TEXTURE_FORMATS 0x86A2
#define GL_MAJOR_VERSION 0x821B
#define GL_MINOR_VERSION 0x821C
#define GL_NUM_EXTENSIONS 0x821D
#define GL_NUM_SAMPLE_COUNTS 0x9380
#define GL_PRIMITIVE_RESTART_FIXED_INDEX 0x8D69
#define GL_TEXTURE_SWIZZLE_R 0x8E42
#define GL_TEXTURE_SWIZZLE_G 0x8E43
#define GL_TEXTURE_SWIZZLE_B 0x8E44
#define GL_TEXTURE_SWIZZLE_A 0x8E45
#define GL_BUFFER_ACCESS_FLAGS 0x911F
#define GL_BUFFER_MAP_LENGTH 0x9120
#define GL_BUFFER_MAP_OFFSET 0x9121
#define GL_BUFFER_MAPPED 0x88BC
#define GL_BUFFER_MAP_POINTER 0x88BD
#define GL_MAP_READ_BIT 0x0001
#define GL_MAP_WRITE_BIT 0x0002
#define GL_MAP_INVALIDATE_RANGE_BIT 0x0004
#define GL_MAP_INVALIDATE_BUFFER_BIT 0x0008
#define GL_MAP_FLUSH_EXPLICIT_BIT 0x0010
#define GL_MAP_UNSYNCHRONIZED_BIT 0x0020
#define GL_COMPRESSED_R11_EAC 0x9270
#define GL_COMPRESSED_SIGNED_R11_EAC 0x9271
#define GL_COMPRESSED_RG11_EAC 0x9272
#define GL_COMPRESSED_SIGNED_RG11_EAC 0x9273
#define GL_COMPRESSED_RGB8_ETC2 0x9274
#define GL_COMPRESSED_SRGB8_ETC2 0x9275
#define GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2 0x9276
#define GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 0x9277
#define GL_COMPRESSED_RGBA8_ETC2_EAC 0x9278
#define GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC 0x9279
#define GL_INVALID_INDEX 0xFFFFFFFFu
#define GL_TIMEOUT_IGNORED 0xFFFFFFFFFFFFFFFFull

/* The numbers of the extensions the loader turns on where the canvas has them (and
   GL_EXT_color_buffer_float, GL_EXT_float_blend, GL_OES_texture_float_linear and
   GL_EXT_render_snorm, which bring none: they let more be drawn into, blended and filtered). */
/* GL_EXT_color_buffer_half_float: half-float pictures drawn into. */
#define GL_RGBA16F_EXT 0x881A
#define GL_RGB16F_EXT 0x881B
#define GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE_EXT 0x8211
#define GL_UNSIGNED_NORMALIZED_EXT 0x8C17
/* GL_EXT_texture_filter_anisotropic: textures seen at a slant, filtered. */
#define GL_TEXTURE_MAX_ANISOTROPY_EXT 0x84FE
#define GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT 0x84FF
/* GL_EXT_texture_norm16: sixteen bits a channel. */
#define GL_R16_EXT 0x822A
#define GL_RG16_EXT 0x822C
#define GL_RGB16_EXT 0x8054
#define GL_RGBA16_EXT 0x805B
#define GL_R16_SNORM_EXT 0x8F98
#define GL_RG16_SNORM_EXT 0x8F99
#define GL_RGB16_SNORM_EXT 0x8F9A
#define GL_RGBA16_SNORM_EXT 0x8F9B
/* GL_EXT_texture_mirror_clamp_to_edge. */
#define GL_MIRROR_CLAMP_TO_EDGE_EXT 0x8743
/* GL_EXT_depth_clamp: glEnable(GL_DEPTH_CLAMP_EXT). */
#define GL_DEPTH_CLAMP_EXT 0x864F
/* GL_EXT_texture_compression_s3tc: DXT1, DXT3 and DXT5. */
#define GL_COMPRESSED_RGB_S3TC_DXT1_EXT 0x83F0
#define GL_COMPRESSED_RGBA_S3TC_DXT1_EXT 0x83F1
#define GL_COMPRESSED_RGBA_S3TC_DXT3_EXT 0x83F2
#define GL_COMPRESSED_RGBA_S3TC_DXT5_EXT 0x83F3
/* GL_EXT_texture_compression_s3tc_srgb. */
#define GL_COMPRESSED_SRGB_S3TC_DXT1_EXT 0x8C4C
#define GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT 0x8C4D
#define GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT 0x8C4E
#define GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT 0x8C4F
/* GL_EXT_texture_compression_rgtc. */
#define GL_COMPRESSED_RED_RGTC1_EXT 0x8DBB
#define GL_COMPRESSED_SIGNED_RED_RGTC1_EXT 0x8DBC
#define GL_COMPRESSED_RED_GREEN_RGTC2_EXT 0x8DBD
#define GL_COMPRESSED_SIGNED_RED_GREEN_RGTC2_EXT 0x8DBE
/* GL_EXT_texture_compression_bptc. */
#define GL_COMPRESSED_RGBA_BPTC_UNORM_EXT 0x8E8C
#define GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_EXT 0x8E8D
#define GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_EXT 0x8E8E
#define GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_EXT 0x8E8F

/* Each call is asked of the loader, by its own name. */
#define GRIDCRAFT_GL(name) __attribute__((import_module("gl"), import_name(#name)))

/* The picture as a whole: what is drawn into it, and how. */
GRIDCRAFT_GL(glViewport) void glViewport(GLint x, GLint y, GLsizei width, GLsizei height);
GRIDCRAFT_GL(glScissor) void glScissor(GLint x, GLint y, GLsizei width, GLsizei height);
GRIDCRAFT_GL(glClearColor) void glClearColor(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
GRIDCRAFT_GL(glClearDepthf) void glClearDepthf(GLfloat d);
GRIDCRAFT_GL(glClearStencil) void glClearStencil(GLint s);
GRIDCRAFT_GL(glClear) void glClear(GLbitfield mask);
GRIDCRAFT_GL(glEnable) void glEnable(GLenum cap);
GRIDCRAFT_GL(glDisable) void glDisable(GLenum cap);
GRIDCRAFT_GL(glIsEnabled) GLboolean glIsEnabled(GLenum cap);
GRIDCRAFT_GL(glDepthFunc) void glDepthFunc(GLenum func);
GRIDCRAFT_GL(glDepthMask) void glDepthMask(GLboolean flag);
GRIDCRAFT_GL(glDepthRangef) void glDepthRangef(GLfloat n, GLfloat f);
GRIDCRAFT_GL(glCullFace) void glCullFace(GLenum mode);
GRIDCRAFT_GL(glFrontFace) void glFrontFace(GLenum mode);
GRIDCRAFT_GL(glBlendFunc) void glBlendFunc(GLenum sfactor, GLenum dfactor);
GRIDCRAFT_GL(glBlendFuncSeparate) void glBlendFuncSeparate(GLenum sfactorRGB, GLenum dfactorRGB, GLenum sfactorAlpha, GLenum dfactorAlpha);
GRIDCRAFT_GL(glBlendEquation) void glBlendEquation(GLenum mode);
GRIDCRAFT_GL(glBlendEquationSeparate) void glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha);
GRIDCRAFT_GL(glBlendColor) void glBlendColor(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
GRIDCRAFT_GL(glColorMask) void glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
GRIDCRAFT_GL(glPolygonOffset) void glPolygonOffset(GLfloat factor, GLfloat units);
GRIDCRAFT_GL(glLineWidth) void glLineWidth(GLfloat width);
GRIDCRAFT_GL(glSampleCoverage) void glSampleCoverage(GLfloat value, GLboolean invert);
GRIDCRAFT_GL(glStencilFunc) void glStencilFunc(GLenum func, GLint ref, GLuint mask);
GRIDCRAFT_GL(glStencilFuncSeparate) void glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask);
GRIDCRAFT_GL(glStencilOp) void glStencilOp(GLenum fail, GLenum zfail, GLenum zpass);
GRIDCRAFT_GL(glStencilOpSeparate) void glStencilOpSeparate(GLenum face, GLenum sfail, GLenum dpfail, GLenum dppass);
GRIDCRAFT_GL(glStencilMask) void glStencilMask(GLuint mask);
GRIDCRAFT_GL(glStencilMaskSeparate) void glStencilMaskSeparate(GLenum face, GLuint mask);
GRIDCRAFT_GL(glHint) void glHint(GLenum target, GLenum mode);
GRIDCRAFT_GL(glPixelStorei) void glPixelStorei(GLenum pname, GLint param);
GRIDCRAFT_GL(glReadPixels) void glReadPixels(GLint x, GLint y, GLsizei width, GLsizei height, GLenum format, GLenum type, void *pixels);
GRIDCRAFT_GL(glReadBuffer) void glReadBuffer(GLenum src);
GRIDCRAFT_GL(glFinish) void glFinish(void);
GRIDCRAFT_GL(glFlush) void glFlush(void);
GRIDCRAFT_GL(glGetError) GLenum glGetError(void);

/* What OpenGL is asked of itself. */
GRIDCRAFT_GL(glGetBooleanv) void glGetBooleanv(GLenum pname, GLboolean *data);
GRIDCRAFT_GL(glGetFloatv) void glGetFloatv(GLenum pname, GLfloat *data);
GRIDCRAFT_GL(glGetIntegerv) void glGetIntegerv(GLenum pname, GLint *data);
GRIDCRAFT_GL(glGetInteger64v) void glGetInteger64v(GLenum pname, GLint64 *data);
GRIDCRAFT_GL(glGetIntegeri_v) void glGetIntegeri_v(GLenum target, GLuint index, GLint *data);
GRIDCRAFT_GL(glGetInteger64i_v) void glGetInteger64i_v(GLenum target, GLuint index, GLint64 *data);
GRIDCRAFT_GL(glGetInternalformativ) void glGetInternalformativ(GLenum target, GLenum internalformat, GLenum pname, GLsizei bufSize, GLint *params);

/* Shaders, and the programs made of them. */
GRIDCRAFT_GL(glCreateShader) GLuint glCreateShader(GLenum type);
GRIDCRAFT_GL(glShaderSource) void glShaderSource(GLuint shader, GLsizei count, const GLchar *const *string, const GLint *length);
GRIDCRAFT_GL(glCompileShader) void glCompileShader(GLuint shader);
GRIDCRAFT_GL(glGetShaderiv) void glGetShaderiv(GLuint shader, GLenum pname, GLint *params);
GRIDCRAFT_GL(glGetShaderInfoLog) void glGetShaderInfoLog(GLuint shader, GLsizei bufSize, GLsizei *length, GLchar *infoLog);
GRIDCRAFT_GL(glGetShaderSource) void glGetShaderSource(GLuint shader, GLsizei bufSize, GLsizei *length, GLchar *source);
GRIDCRAFT_GL(glGetShaderPrecisionFormat) void glGetShaderPrecisionFormat(GLenum shadertype, GLenum precisiontype, GLint *range, GLint *precision);
GRIDCRAFT_GL(glDeleteShader) void glDeleteShader(GLuint shader);
GRIDCRAFT_GL(glIsShader) GLboolean glIsShader(GLuint shader);
GRIDCRAFT_GL(glReleaseShaderCompiler) void glReleaseShaderCompiler(void);
GRIDCRAFT_GL(glShaderBinary) void glShaderBinary(GLsizei count, const GLuint *shaders, GLenum binaryFormat, const void *binary, GLsizei length);
GRIDCRAFT_GL(glCreateProgram) GLuint glCreateProgram(void);
GRIDCRAFT_GL(glAttachShader) void glAttachShader(GLuint program, GLuint shader);
GRIDCRAFT_GL(glDetachShader) void glDetachShader(GLuint program, GLuint shader);
GRIDCRAFT_GL(glGetAttachedShaders) void glGetAttachedShaders(GLuint program, GLsizei maxCount, GLsizei *count, GLuint *shaders);
GRIDCRAFT_GL(glBindAttribLocation) void glBindAttribLocation(GLuint program, GLuint index, const GLchar *name);
GRIDCRAFT_GL(glLinkProgram) void glLinkProgram(GLuint program);
GRIDCRAFT_GL(glValidateProgram) void glValidateProgram(GLuint program);
GRIDCRAFT_GL(glGetProgramiv) void glGetProgramiv(GLuint program, GLenum pname, GLint *params);
GRIDCRAFT_GL(glGetProgramInfoLog) void glGetProgramInfoLog(GLuint program, GLsizei bufSize, GLsizei *length, GLchar *infoLog);
GRIDCRAFT_GL(glUseProgram) void glUseProgram(GLuint program);
GRIDCRAFT_GL(glDeleteProgram) void glDeleteProgram(GLuint program);
GRIDCRAFT_GL(glIsProgram) GLboolean glIsProgram(GLuint program);
GRIDCRAFT_GL(glGetProgramBinary) void glGetProgramBinary(GLuint program, GLsizei bufSize, GLsizei *length, GLenum *binaryFormat, void *binary);
GRIDCRAFT_GL(glProgramBinary) void glProgramBinary(GLuint program, GLenum binaryFormat, const void *binary, GLsizei length);
GRIDCRAFT_GL(glProgramParameteri) void glProgramParameteri(GLuint program, GLenum pname, GLint value);
GRIDCRAFT_GL(glGetAttribLocation) GLint glGetAttribLocation(GLuint program, const GLchar *name);
GRIDCRAFT_GL(glGetActiveAttrib) void glGetActiveAttrib(GLuint program, GLuint index, GLsizei bufSize, GLsizei *length, GLint *size, GLenum *type, GLchar *name);
GRIDCRAFT_GL(glGetFragDataLocation) GLint glGetFragDataLocation(GLuint program, const GLchar *name);
GRIDCRAFT_GL(glTransformFeedbackVaryings) void glTransformFeedbackVaryings(GLuint program, GLsizei count, const GLchar *const *varyings, GLenum bufferMode);
GRIDCRAFT_GL(glGetTransformFeedbackVarying) void glGetTransformFeedbackVarying(GLuint program, GLuint index, GLsizei bufSize, GLsizei *length, GLsizei *size, GLenum *type, GLchar *name);

/* What a shader is told: uniforms, one at a time and in blocks. */
GRIDCRAFT_GL(glGetUniformLocation) GLint glGetUniformLocation(GLuint program, const GLchar *name);
GRIDCRAFT_GL(glGetActiveUniform) void glGetActiveUniform(GLuint program, GLuint index, GLsizei bufSize, GLsizei *length, GLint *size, GLenum *type, GLchar *name);
GRIDCRAFT_GL(glGetUniformfv) void glGetUniformfv(GLuint program, GLint location, GLfloat *params);
GRIDCRAFT_GL(glGetUniformiv) void glGetUniformiv(GLuint program, GLint location, GLint *params);
GRIDCRAFT_GL(glGetUniformuiv) void glGetUniformuiv(GLuint program, GLint location, GLuint *params);
GRIDCRAFT_GL(glUniform1f) void glUniform1f(GLint location, GLfloat v0);
GRIDCRAFT_GL(glUniform2f) void glUniform2f(GLint location, GLfloat v0, GLfloat v1);
GRIDCRAFT_GL(glUniform3f) void glUniform3f(GLint location, GLfloat v0, GLfloat v1, GLfloat v2);
GRIDCRAFT_GL(glUniform4f) void glUniform4f(GLint location, GLfloat v0, GLfloat v1, GLfloat v2, GLfloat v3);
GRIDCRAFT_GL(glUniform1i) void glUniform1i(GLint location, GLint v0);
GRIDCRAFT_GL(glUniform2i) void glUniform2i(GLint location, GLint v0, GLint v1);
GRIDCRAFT_GL(glUniform3i) void glUniform3i(GLint location, GLint v0, GLint v1, GLint v2);
GRIDCRAFT_GL(glUniform4i) void glUniform4i(GLint location, GLint v0, GLint v1, GLint v2, GLint v3);
GRIDCRAFT_GL(glUniform1ui) void glUniform1ui(GLint location, GLuint v0);
GRIDCRAFT_GL(glUniform2ui) void glUniform2ui(GLint location, GLuint v0, GLuint v1);
GRIDCRAFT_GL(glUniform3ui) void glUniform3ui(GLint location, GLuint v0, GLuint v1, GLuint v2);
GRIDCRAFT_GL(glUniform4ui) void glUniform4ui(GLint location, GLuint v0, GLuint v1, GLuint v2, GLuint v3);
GRIDCRAFT_GL(glUniform1fv) void glUniform1fv(GLint location, GLsizei count, const GLfloat *value);
GRIDCRAFT_GL(glUniform2fv) void glUniform2fv(GLint location, GLsizei count, const GLfloat *value);
GRIDCRAFT_GL(glUniform3fv) void glUniform3fv(GLint location, GLsizei count, const GLfloat *value);
GRIDCRAFT_GL(glUniform4fv) void glUniform4fv(GLint location, GLsizei count, const GLfloat *value);
GRIDCRAFT_GL(glUniform1iv) void glUniform1iv(GLint location, GLsizei count, const GLint *value);
GRIDCRAFT_GL(glUniform2iv) void glUniform2iv(GLint location, GLsizei count, const GLint *value);
GRIDCRAFT_GL(glUniform3iv) void glUniform3iv(GLint location, GLsizei count, const GLint *value);
GRIDCRAFT_GL(glUniform4iv) void glUniform4iv(GLint location, GLsizei count, const GLint *value);
GRIDCRAFT_GL(glUniform1uiv) void glUniform1uiv(GLint location, GLsizei count, const GLuint *value);
GRIDCRAFT_GL(glUniform2uiv) void glUniform2uiv(GLint location, GLsizei count, const GLuint *value);
GRIDCRAFT_GL(glUniform3uiv) void glUniform3uiv(GLint location, GLsizei count, const GLuint *value);
GRIDCRAFT_GL(glUniform4uiv) void glUniform4uiv(GLint location, GLsizei count, const GLuint *value);
GRIDCRAFT_GL(glUniformMatrix2fv) void glUniformMatrix2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
GRIDCRAFT_GL(glUniformMatrix3fv) void glUniformMatrix3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
GRIDCRAFT_GL(glUniformMatrix4fv) void glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
GRIDCRAFT_GL(glUniformMatrix2x3fv) void glUniformMatrix2x3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
GRIDCRAFT_GL(glUniformMatrix3x2fv) void glUniformMatrix3x2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
GRIDCRAFT_GL(glUniformMatrix2x4fv) void glUniformMatrix2x4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
GRIDCRAFT_GL(glUniformMatrix4x2fv) void glUniformMatrix4x2fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
GRIDCRAFT_GL(glUniformMatrix3x4fv) void glUniformMatrix3x4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
GRIDCRAFT_GL(glUniformMatrix4x3fv) void glUniformMatrix4x3fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat *value);
GRIDCRAFT_GL(glGetUniformBlockIndex) GLuint glGetUniformBlockIndex(GLuint program, const GLchar *uniformBlockName);
GRIDCRAFT_GL(glGetActiveUniformBlockiv) void glGetActiveUniformBlockiv(GLuint program, GLuint uniformBlockIndex, GLenum pname, GLint *params);
GRIDCRAFT_GL(glGetActiveUniformBlockName) void glGetActiveUniformBlockName(GLuint program, GLuint uniformBlockIndex, GLsizei bufSize, GLsizei *length, GLchar *uniformBlockName);
GRIDCRAFT_GL(glUniformBlockBinding) void glUniformBlockBinding(GLuint program, GLuint uniformBlockIndex, GLuint uniformBlockBinding);
GRIDCRAFT_GL(glGetUniformIndices) void glGetUniformIndices(GLuint program, GLsizei uniformCount, const GLchar *const *uniformNames, GLuint *uniformIndices);
GRIDCRAFT_GL(glGetActiveUniformsiv) void glGetActiveUniformsiv(GLuint program, GLsizei uniformCount, const GLuint *uniformIndices, GLenum pname, GLint *params);

/* Buffers of numbers. (Mapping one is done by the kit's C, below.) */
GRIDCRAFT_GL(glGenBuffers) void glGenBuffers(GLsizei n, GLuint *buffers);
GRIDCRAFT_GL(glDeleteBuffers) void glDeleteBuffers(GLsizei n, const GLuint *buffers);
GRIDCRAFT_GL(glIsBuffer) GLboolean glIsBuffer(GLuint buffer);
GRIDCRAFT_GL(glBindBuffer) void glBindBuffer(GLenum target, GLuint buffer);
GRIDCRAFT_GL(glBindBufferBase) void glBindBufferBase(GLenum target, GLuint index, GLuint buffer);
GRIDCRAFT_GL(glBindBufferRange) void glBindBufferRange(GLenum target, GLuint index, GLuint buffer, GLintptr offset, GLsizeiptr size);
GRIDCRAFT_GL(glBufferData) void glBufferData(GLenum target, GLsizeiptr size, const void *data, GLenum usage);
GRIDCRAFT_GL(glBufferSubData) void glBufferSubData(GLenum target, GLintptr offset, GLsizeiptr size, const void *data);
GRIDCRAFT_GL(glCopyBufferSubData) void glCopyBufferSubData(GLenum readTarget, GLenum writeTarget, GLintptr readOffset, GLintptr writeOffset, GLsizeiptr size);
GRIDCRAFT_GL(glGetBufferParameteriv) void glGetBufferParameteriv(GLenum target, GLenum pname, GLint *params);
GRIDCRAFT_GL(glGetBufferParameteri64v) void glGetBufferParameteri64v(GLenum target, GLenum pname, GLint64 *params);
GRIDCRAFT_GL(glGetBufferPointerv) void glGetBufferPointerv(GLenum target, GLenum pname, void **params);

/* What is drawn: how the numbers are read, and drawing, once or many times over. */
GRIDCRAFT_GL(glGenVertexArrays) void glGenVertexArrays(GLsizei n, GLuint *arrays);
GRIDCRAFT_GL(glDeleteVertexArrays) void glDeleteVertexArrays(GLsizei n, const GLuint *arrays);
GRIDCRAFT_GL(glBindVertexArray) void glBindVertexArray(GLuint array);
GRIDCRAFT_GL(glIsVertexArray) GLboolean glIsVertexArray(GLuint array);
GRIDCRAFT_GL(glEnableVertexAttribArray) void glEnableVertexAttribArray(GLuint index);
GRIDCRAFT_GL(glDisableVertexAttribArray) void glDisableVertexAttribArray(GLuint index);
GRIDCRAFT_GL(glVertexAttribPointer) void glVertexAttribPointer(GLuint index, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void *pointer);
GRIDCRAFT_GL(glVertexAttribIPointer) void glVertexAttribIPointer(GLuint index, GLint size, GLenum type, GLsizei stride, const void *pointer);
GRIDCRAFT_GL(glVertexAttribDivisor) void glVertexAttribDivisor(GLuint index, GLuint divisor);
GRIDCRAFT_GL(glVertexAttrib1f) void glVertexAttrib1f(GLuint index, GLfloat x);
GRIDCRAFT_GL(glVertexAttrib2f) void glVertexAttrib2f(GLuint index, GLfloat x, GLfloat y);
GRIDCRAFT_GL(glVertexAttrib3f) void glVertexAttrib3f(GLuint index, GLfloat x, GLfloat y, GLfloat z);
GRIDCRAFT_GL(glVertexAttrib4f) void glVertexAttrib4f(GLuint index, GLfloat x, GLfloat y, GLfloat z, GLfloat w);
GRIDCRAFT_GL(glVertexAttrib1fv) void glVertexAttrib1fv(GLuint index, const GLfloat *v);
GRIDCRAFT_GL(glVertexAttrib2fv) void glVertexAttrib2fv(GLuint index, const GLfloat *v);
GRIDCRAFT_GL(glVertexAttrib3fv) void glVertexAttrib3fv(GLuint index, const GLfloat *v);
GRIDCRAFT_GL(glVertexAttrib4fv) void glVertexAttrib4fv(GLuint index, const GLfloat *v);
GRIDCRAFT_GL(glVertexAttribI4i) void glVertexAttribI4i(GLuint index, GLint x, GLint y, GLint z, GLint w);
GRIDCRAFT_GL(glVertexAttribI4ui) void glVertexAttribI4ui(GLuint index, GLuint x, GLuint y, GLuint z, GLuint w);
GRIDCRAFT_GL(glVertexAttribI4iv) void glVertexAttribI4iv(GLuint index, const GLint *v);
GRIDCRAFT_GL(glVertexAttribI4uiv) void glVertexAttribI4uiv(GLuint index, const GLuint *v);
GRIDCRAFT_GL(glGetVertexAttribfv) void glGetVertexAttribfv(GLuint index, GLenum pname, GLfloat *params);
GRIDCRAFT_GL(glGetVertexAttribiv) void glGetVertexAttribiv(GLuint index, GLenum pname, GLint *params);
GRIDCRAFT_GL(glGetVertexAttribIiv) void glGetVertexAttribIiv(GLuint index, GLenum pname, GLint *params);
GRIDCRAFT_GL(glGetVertexAttribIuiv) void glGetVertexAttribIuiv(GLuint index, GLenum pname, GLuint *params);
GRIDCRAFT_GL(glGetVertexAttribPointerv) void glGetVertexAttribPointerv(GLuint index, GLenum pname, void **pointer);
GRIDCRAFT_GL(glDrawArrays) void glDrawArrays(GLenum mode, GLint first, GLsizei count);
GRIDCRAFT_GL(glDrawElements) void glDrawElements(GLenum mode, GLsizei count, GLenum type, const void *indices);
GRIDCRAFT_GL(glDrawRangeElements) void glDrawRangeElements(GLenum mode, GLuint start, GLuint end, GLsizei count, GLenum type, const void *indices);
GRIDCRAFT_GL(glDrawArraysInstanced) void glDrawArraysInstanced(GLenum mode, GLint first, GLsizei count, GLsizei instancecount);
GRIDCRAFT_GL(glDrawElementsInstanced) void glDrawElementsInstanced(GLenum mode, GLsizei count, GLenum type, const void *indices, GLsizei instancecount);

/* Pictures to draw with: textures, flat, layered and deep. */
GRIDCRAFT_GL(glGenTextures) void glGenTextures(GLsizei n, GLuint *textures);
GRIDCRAFT_GL(glDeleteTextures) void glDeleteTextures(GLsizei n, const GLuint *textures);
GRIDCRAFT_GL(glBindTexture) void glBindTexture(GLenum target, GLuint texture);
GRIDCRAFT_GL(glIsTexture) GLboolean glIsTexture(GLuint texture);
GRIDCRAFT_GL(glActiveTexture) void glActiveTexture(GLenum texture);
GRIDCRAFT_GL(glTexImage2D) void glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const void *pixels);
GRIDCRAFT_GL(glTexImage3D) void glTexImage3D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLsizei depth, GLint border, GLenum format, GLenum type, const void *pixels);
GRIDCRAFT_GL(glTexSubImage2D) void glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const void *pixels);
GRIDCRAFT_GL(glTexSubImage3D) void glTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLenum type, const void *pixels);
GRIDCRAFT_GL(glTexStorage2D) void glTexStorage2D(GLenum target, GLsizei levels, GLenum internalformat, GLsizei width, GLsizei height);
GRIDCRAFT_GL(glTexStorage3D) void glTexStorage3D(GLenum target, GLsizei levels, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth);
GRIDCRAFT_GL(glCopyTexImage2D) void glCopyTexImage2D(GLenum target, GLint level, GLenum internalformat, GLint x, GLint y, GLsizei width, GLsizei height, GLint border);
GRIDCRAFT_GL(glCopyTexSubImage2D) void glCopyTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint x, GLint y, GLsizei width, GLsizei height);
GRIDCRAFT_GL(glCopyTexSubImage3D) void glCopyTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLint x, GLint y, GLsizei width, GLsizei height);
GRIDCRAFT_GL(glCompressedTexImage2D) void glCompressedTexImage2D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void *data);
GRIDCRAFT_GL(glCompressedTexImage3D) void glCompressedTexImage3D(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth, GLint border, GLsizei imageSize, const void *data);
GRIDCRAFT_GL(glCompressedTexSubImage2D) void glCompressedTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLsizei imageSize, const void *data);
GRIDCRAFT_GL(glCompressedTexSubImage3D) void glCompressedTexSubImage3D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLsizei imageSize, const void *data);
GRIDCRAFT_GL(glTexParameterf) void glTexParameterf(GLenum target, GLenum pname, GLfloat param);
GRIDCRAFT_GL(glTexParameterfv) void glTexParameterfv(GLenum target, GLenum pname, const GLfloat *params);
GRIDCRAFT_GL(glTexParameteri) void glTexParameteri(GLenum target, GLenum pname, GLint param);
GRIDCRAFT_GL(glTexParameteriv) void glTexParameteriv(GLenum target, GLenum pname, const GLint *params);
GRIDCRAFT_GL(glGetTexParameterfv) void glGetTexParameterfv(GLenum target, GLenum pname, GLfloat *params);
GRIDCRAFT_GL(glGetTexParameteriv) void glGetTexParameteriv(GLenum target, GLenum pname, GLint *params);
GRIDCRAFT_GL(glGenerateMipmap) void glGenerateMipmap(GLenum target);

/* How a texture is read, apart from the texture: samplers. */
GRIDCRAFT_GL(glGenSamplers) void glGenSamplers(GLsizei count, GLuint *samplers);
GRIDCRAFT_GL(glDeleteSamplers) void glDeleteSamplers(GLsizei count, const GLuint *samplers);
GRIDCRAFT_GL(glIsSampler) GLboolean glIsSampler(GLuint sampler);
GRIDCRAFT_GL(glBindSampler) void glBindSampler(GLuint unit, GLuint sampler);
GRIDCRAFT_GL(glSamplerParameteri) void glSamplerParameteri(GLuint sampler, GLenum pname, GLint param);
GRIDCRAFT_GL(glSamplerParameteriv) void glSamplerParameteriv(GLuint sampler, GLenum pname, const GLint *param);
GRIDCRAFT_GL(glSamplerParameterf) void glSamplerParameterf(GLuint sampler, GLenum pname, GLfloat param);
GRIDCRAFT_GL(glSamplerParameterfv) void glSamplerParameterfv(GLuint sampler, GLenum pname, const GLfloat *param);
GRIDCRAFT_GL(glGetSamplerParameteriv) void glGetSamplerParameteriv(GLuint sampler, GLenum pname, GLint *params);
GRIDCRAFT_GL(glGetSamplerParameterfv) void glGetSamplerParameterfv(GLuint sampler, GLenum pname, GLfloat *params);

/* Pictures drawn into: framebuffers, and the renderbuffers and textures they draw into. */
GRIDCRAFT_GL(glGenFramebuffers) void glGenFramebuffers(GLsizei n, GLuint *framebuffers);
GRIDCRAFT_GL(glDeleteFramebuffers) void glDeleteFramebuffers(GLsizei n, const GLuint *framebuffers);
GRIDCRAFT_GL(glIsFramebuffer) GLboolean glIsFramebuffer(GLuint framebuffer);
GRIDCRAFT_GL(glBindFramebuffer) void glBindFramebuffer(GLenum target, GLuint framebuffer);
GRIDCRAFT_GL(glCheckFramebufferStatus) GLenum glCheckFramebufferStatus(GLenum target);
GRIDCRAFT_GL(glFramebufferTexture2D) void glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
GRIDCRAFT_GL(glFramebufferTextureLayer) void glFramebufferTextureLayer(GLenum target, GLenum attachment, GLuint texture, GLint level, GLint layer);
GRIDCRAFT_GL(glFramebufferRenderbuffer) void glFramebufferRenderbuffer(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
GRIDCRAFT_GL(glGetFramebufferAttachmentParameteriv) void glGetFramebufferAttachmentParameteriv(GLenum target, GLenum attachment, GLenum pname, GLint *params);
GRIDCRAFT_GL(glDrawBuffers) void glDrawBuffers(GLsizei n, const GLenum *bufs);
GRIDCRAFT_GL(glBlitFramebuffer) void glBlitFramebuffer(GLint srcX0, GLint srcY0, GLint srcX1, GLint srcY1, GLint dstX0, GLint dstY0, GLint dstX1, GLint dstY1, GLbitfield mask, GLenum filter);
GRIDCRAFT_GL(glInvalidateFramebuffer) void glInvalidateFramebuffer(GLenum target, GLsizei numAttachments, const GLenum *attachments);
GRIDCRAFT_GL(glInvalidateSubFramebuffer) void glInvalidateSubFramebuffer(GLenum target, GLsizei numAttachments, const GLenum *attachments, GLint x, GLint y, GLsizei width, GLsizei height);
GRIDCRAFT_GL(glClearBufferiv) void glClearBufferiv(GLenum buffer, GLint drawbuffer, const GLint *value);
GRIDCRAFT_GL(glClearBufferuiv) void glClearBufferuiv(GLenum buffer, GLint drawbuffer, const GLuint *value);
GRIDCRAFT_GL(glClearBufferfv) void glClearBufferfv(GLenum buffer, GLint drawbuffer, const GLfloat *value);
GRIDCRAFT_GL(glClearBufferfi) void glClearBufferfi(GLenum buffer, GLint drawbuffer, GLfloat depth, GLint stencil);
GRIDCRAFT_GL(glGenRenderbuffers) void glGenRenderbuffers(GLsizei n, GLuint *renderbuffers);
GRIDCRAFT_GL(glDeleteRenderbuffers) void glDeleteRenderbuffers(GLsizei n, const GLuint *renderbuffers);
GRIDCRAFT_GL(glIsRenderbuffer) GLboolean glIsRenderbuffer(GLuint renderbuffer);
GRIDCRAFT_GL(glBindRenderbuffer) void glBindRenderbuffer(GLenum target, GLuint renderbuffer);
GRIDCRAFT_GL(glRenderbufferStorage) void glRenderbufferStorage(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
GRIDCRAFT_GL(glRenderbufferStorageMultisample) void glRenderbufferStorageMultisample(GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height);
GRIDCRAFT_GL(glGetRenderbufferParameteriv) void glGetRenderbufferParameteriv(GLenum target, GLenum pname, GLint *params);

/* Questions asked of the drawing: queries. (A page is told their answers a while later.) */
GRIDCRAFT_GL(glGenQueries) void glGenQueries(GLsizei n, GLuint *ids);
GRIDCRAFT_GL(glDeleteQueries) void glDeleteQueries(GLsizei n, const GLuint *ids);
GRIDCRAFT_GL(glIsQuery) GLboolean glIsQuery(GLuint id);
GRIDCRAFT_GL(glBeginQuery) void glBeginQuery(GLenum target, GLuint id);
GRIDCRAFT_GL(glEndQuery) void glEndQuery(GLenum target);
GRIDCRAFT_GL(glGetQueryiv) void glGetQueryiv(GLenum target, GLenum pname, GLint *params);
GRIDCRAFT_GL(glGetQueryObjectuiv) void glGetQueryObjectuiv(GLuint id, GLenum pname, GLuint *params);

/* What a vertex shader works out, kept in buffers: transform feedback. */
GRIDCRAFT_GL(glGenTransformFeedbacks) void glGenTransformFeedbacks(GLsizei n, GLuint *ids);
GRIDCRAFT_GL(glDeleteTransformFeedbacks) void glDeleteTransformFeedbacks(GLsizei n, const GLuint *ids);
GRIDCRAFT_GL(glIsTransformFeedback) GLboolean glIsTransformFeedback(GLuint id);
GRIDCRAFT_GL(glBindTransformFeedback) void glBindTransformFeedback(GLenum target, GLuint id);
GRIDCRAFT_GL(glBeginTransformFeedback) void glBeginTransformFeedback(GLenum primitiveMode);
GRIDCRAFT_GL(glEndTransformFeedback) void glEndTransformFeedback(void);
GRIDCRAFT_GL(glPauseTransformFeedback) void glPauseTransformFeedback(void);
GRIDCRAFT_GL(glResumeTransformFeedback) void glResumeTransformFeedback(void);

/* Fences: whether the drawing asked for so far is done. (A page may not wait long for one.) */
GRIDCRAFT_GL(glFenceSync) GLsync glFenceSync(GLenum condition, GLbitfield flags);
GRIDCRAFT_GL(glIsSync) GLboolean glIsSync(GLsync sync);
GRIDCRAFT_GL(glDeleteSync) void glDeleteSync(GLsync sync);
GRIDCRAFT_GL(glClientWaitSync) GLenum glClientWaitSync(GLsync sync, GLbitfield flags, GLuint64 timeout);
GRIDCRAFT_GL(glWaitSync) void glWaitSync(GLsync sync, GLbitfield flags, GLuint64 timeout);
GRIDCRAFT_GL(glGetSynciv) void glGetSynciv(GLsync sync, GLenum pname, GLsizei bufSize, GLsizei *length, GLint *values);

#undef GRIDCRAFT_GL

/* Done in the program, by the kit's own C, which is built into a program that includes this:
   these hand back a pointer into the program's own memory. A mapped buffer is a copy in the
   program's memory, read from the buffer when it is mapped for reading (or when it is not to be
   thrown away) and written back when it is unmapped, or flushed. */
const GLubyte *glGetString(GLenum name);
const GLubyte *glGetStringi(GLenum name, GLuint index);
void *glMapBufferRange(GLenum target, GLintptr offset, GLsizeiptr length, GLbitfield access);
GLboolean glUnmapBuffer(GLenum target);
void glFlushMappedBufferRange(GLenum target, GLintptr offset, GLsizeiptr length);

#ifdef __cplusplus
}
#endif

#endif
"#;

/// The kit's C for the calls that hand back a pointer, built into every program that includes
/// [`HEADER`].
pub const SOURCE: &str = r#"/*
 * GridCraft's C Development Kit: the part of OpenGL ES 3.0 that hands a program a pointer into its
 * own memory, done in the program, since the loader has no memory of the program's to hand.
 * Built into every program that includes <GLES3/gl3.h>. The file is GridCraft's, written for this
 * kit, and is not a project's to change.
 */
#include <GLES3/gl3.h>
#include <stdlib.h>
#include <string.h>

/* What the loader answers for these, in the program's own memory. */
#define GRIDCRAFT_KIT(name) __attribute__((import_module("gl"), import_name(#name)))
GRIDCRAFT_KIT(gridcraft_glString) GLint gridcraft_glString(GLenum name, GLint index, GLchar *into, GLsizei room);
GRIDCRAFT_KIT(gridcraft_glMap) GLenum gridcraft_glMap(GLenum target, GLintptr offset, GLsizeiptr length, GLbitfield access, void *at);
GRIDCRAFT_KIT(gridcraft_glMapped) GLint gridcraft_glMapped(GLenum target, GLint *into);
GRIDCRAFT_KIT(gridcraft_glUnmap) void gridcraft_glUnmap(GLenum target);
GRIDCRAFT_KIT(gridcraft_glReadBuffer) void gridcraft_glReadBuffer(GLenum target, GLintptr offset, GLsizeiptr size, void *into);
GRIDCRAFT_KIT(gridcraft_glFail) void gridcraft_glFail(GLenum error);
#undef GRIDCRAFT_KIT

/* What OpenGL says of itself, each kept where it was first written, as OpenGL keeps it. */
static const GLenum SAID_NAMES[] = {GL_VENDOR, GL_RENDERER, GL_VERSION, GL_SHADING_LANGUAGE_VERSION, GL_EXTENSIONS};
static GLchar said[5][4096];
static GLboolean asked[5];
static GLchar extension_names[64][96];
static GLboolean extension_asked[64];

const GLubyte *glGetString(GLenum name)
{
    for (unsigned i = 0; i < sizeof SAID_NAMES / sizeof *SAID_NAMES; i++) {
        if (SAID_NAMES[i] != name)
            continue;
        if (!asked[i] && gridcraft_glString(name, -1, said[i], (GLsizei) sizeof said[i]) < 0)
            return 0;
        asked[i] = GL_TRUE;
        return (const GLubyte *) said[i];
    }
    gridcraft_glString(name, -1, 0, 0);
    return 0;
}

const GLubyte *glGetStringi(GLenum name, GLuint index)
{
    if (name != GL_EXTENSIONS || index >= sizeof extension_names / sizeof *extension_names) {
        gridcraft_glString(name, (GLint) index, 0, 0);
        return 0;
    }
    if (!extension_asked[index] && gridcraft_glString(name, (GLint) index, extension_names[index], (GLsizei) sizeof extension_names[index]) < 0)
        return 0;
    extension_asked[index] = GL_TRUE;
    return (const GLubyte *) extension_names[index];
}

void *glMapBufferRange(GLenum target, GLintptr offset, GLsizeiptr length, GLbitfield access)
{
    void *at;
    GLenum refused;

    if (offset < 0 || length <= 0 || !(access & (GL_MAP_READ_BIT | GL_MAP_WRITE_BIT))) {
        gridcraft_glFail(GL_INVALID_VALUE);
        return 0;
    }
    at = malloc((size_t) length);
    if (!at) {
        gridcraft_glFail(GL_OUT_OF_MEMORY);
        return 0;
    }
    refused = gridcraft_glMap(target, offset, length, access, at);
    if (refused) {
        free(at);
        gridcraft_glFail(refused);
        return 0;
    }
    /* Read for reading, and for writing a part of it: the rest is written back as it was. */
    if ((access & GL_MAP_READ_BIT) || !(access & (GL_MAP_INVALIDATE_RANGE_BIT | GL_MAP_INVALIDATE_BUFFER_BIT)))
        gridcraft_glReadBuffer(target, offset, length, at);
    return at;
}

GLboolean glUnmapBuffer(GLenum target)
{
    GLint mapped[4]; /* offset, length, access, where */

    if (!gridcraft_glMapped(target, mapped)) {
        gridcraft_glFail(GL_INVALID_OPERATION);
        return GL_FALSE;
    }
    if ((mapped[2] & GL_MAP_WRITE_BIT) && !(mapped[2] & GL_MAP_FLUSH_EXPLICIT_BIT))
        glBufferSubData(target, mapped[0], mapped[1], (const void *) (GLintptr) mapped[3]);
    gridcraft_glUnmap(target);
    free((void *) (GLintptr) mapped[3]);
    return GL_TRUE;
}

void glFlushMappedBufferRange(GLenum target, GLintptr offset, GLsizeiptr length)
{
    GLint mapped[4];

    if (!gridcraft_glMapped(target, mapped) || !(mapped[2] & GL_MAP_FLUSH_EXPLICIT_BIT)) {
        gridcraft_glFail(GL_INVALID_OPERATION);
        return;
    }
    if (offset < 0 || length < 0 || offset + length > mapped[1]) {
        gridcraft_glFail(GL_INVALID_VALUE);
        return;
    }
    glBufferSubData(target, mapped[0] + offset, length, (const char *) (GLintptr) mapped[3] + offset);
}
"#;

/// The calls the header declares that the loader answers, by name.
pub fn declared() -> Vec<&'static str> {
    HEADER.lines().filter_map(|line| line.strip_prefix("GRIDCRAFT_GL(")?.split(')').next()).collect()
}

/// What the kit's C asks of the loader, by name.
pub fn asked_by_the_kit() -> Vec<&'static str> {
    SOURCE.lines().filter_map(|line| line.strip_prefix("GRIDCRAFT_KIT(")?.split(')').next()).filter(|name| *name != "name").collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_header_declares_the_whole_of_opengl_es_3() {
        let calls = declared();
        assert_eq!(calls.len(), 241, "every call of OpenGL ES 3.0 that the loader answers");
        let mut unique = calls.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(unique.len(), calls.len(), "each once");
        // What a first program draws with, and what OpenGL ES 3.0 added.
        for call in [
            "glDrawArrays",
            "glDrawArraysInstanced",
            "glVertexAttribDivisor",
            "glBindFramebuffer",
            "glRenderbufferStorageMultisample",
            "glBlitFramebuffer",
            "glDrawBuffers",
            "glBindBufferBase",
            "glUniformBlockBinding",
            "glTexImage3D",
            "glTexStorage2D",
            "glGenSamplers",
            "glBeginQuery",
            "glBeginTransformFeedback",
            "glFenceSync",
            "glClientWaitSync",
        ] {
            assert!(calls.contains(&call), "{call}");
        }
        // The five that hand back a pointer are the kit's C's, not the loader's.
        for own in ["glGetString", "glGetStringi", "glMapBufferRange", "glUnmapBuffer", "glFlushMappedBufferRange"] {
            assert!(!calls.contains(&own), "{own}");
            assert!(HEADER.contains(&format!("{own}(")) && SOURCE.contains(&format!("{own}(GLenum")), "{own}");
        }
        // Its numbers are OpenGL's: a few of them, as the specification has them.
        for (name, value) in [
            ("GL_TRIANGLES", "0x0004"),
            ("GL_TEXTURE_2D", "0x0DE1"),
            ("GL_FRAMEBUFFER", "0x8D40"),
            ("GL_COLOR_ATTACHMENT0", "0x8CE0"),
            ("GL_UNIFORM_BUFFER", "0x8A11"),
            ("GL_TEXTURE_3D", "0x806F"),
            ("GL_RGBA16F", "0x881A"),
            ("GL_MAP_WRITE_BIT", "0x0002"),
            ("GL_COMPRESSED_RGB8_ETC2", "0x9274"),
            ("GL_TEXTURE_MAX_ANISOTROPY_EXT", "0x84FE"),
        ] {
            assert!(HEADER.contains(&format!("#define {name} {value}\n")), "{name}");
        }
        // Nothing of WebGL's own that a program has no use for.
        assert!(!HEADER.contains("_WEBGL") && !HEADER.contains("WebGLRenderingContext"));
    }

    #[test]
    fn the_kits_c_asks_the_loader_only_under_names_of_the_kits_own() {
        let asked = asked_by_the_kit();
        assert_eq!(
            asked,
            ["gridcraft_glString", "gridcraft_glMap", "gridcraft_glMapped", "gridcraft_glUnmap", "gridcraft_glReadBuffer", "gridcraft_glFail"]
        );
        for name in &asked {
            assert!(!declared().contains(name) && name.starts_with("gridcraft_gl"), "{name}");
        }
    }
}
