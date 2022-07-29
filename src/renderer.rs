extern crate gl;
extern crate glutin;
extern crate libc;

use crate::gameboy::{Gameboy, HEIGHT, WIDTH};
use std::ffi::CString;
use std::iter::repeat;
use std::mem;
use std::ptr;
use std::str;
use std::thread;
use std::time::Duration;

use gl::types::*;
use glutin::event::{ElementState, Event, VirtualKeyCode};
use glutin::window::{Window, WindowBuilder};
use glutin::ContextWrapper;

struct Glcx {
    // gl: gl::Gl,
    tex: GLuint,
    program: GLuint,
    frag: GLuint,
    vert: GLuint,
    ebo: GLuint,
    vbo: GLuint,
    vao: GLuint,
}

pub fn render(mut gameboy: Gameboy) {
    let mut ratio = 1 + (WIDTH / 10);
    let event_loop = glutin::event_loop::EventLoop::new();
    let inner_size = glutin::dpi::LogicalSize {
        width: WIDTH,
        height: HEIGHT,
    };
    let window = glutin::window::WindowBuilder::new()
        .with_title("LR35902")
        .with_inner_size(inner_size)
        // .with_dimensions(glium::glutin::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .with_resizable(false);

    let gl_window = glutin::ContextBuilder::new()
        .build_windowed(window, &event_loop)
        .unwrap();
    let gl_window = unsafe { gl_window.make_current().unwrap() };

    let gl = gl::load_with(|s| gl_window.get_proc_address(s) as *const _);

    // let cx = Glcx::new();

    event_loop.run(move |event, _, control_flow| {
        // for event in window.poll_events() {
        // if !handle(event, &mut gb, &window, &mut ratio, &mut focused) {
        //     break 'outer
        // }
        // }

        // gameboy.frame();
        // cx.draw(gb.image());
        gl_window.swap_buffers().unwrap();
        thread::sleep(Duration::from_millis(10));
    })
}

// fn handle(event: Event, gb: &mut Gb, window: &Window, ratio: &mut u32,
//          focused: &mut bool) -> bool {
//     match event {
//         Event::Closed => return false,
//         Event::Resized(width, height) => {
//             let (width, height) = if width < height {
//                 (width, width * HEIGHT / WIDTH)
//             } else {
//                 (height * WIDTH / HEIGHT, height)
//             };
//             window.set_inner_size(width, height);
//         }
//         Event::Focused(f) => *focused = f,
//         Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Equals)) => {
//             *ratio += 1;
//             window.set_inner_size(WIDTH + 10 * *ratio, HEIGHT + 9 * *ratio);
//         }
//         Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Minus)) => {
//             *ratio -= 1;
//             window.set_inner_size(WIDTH + 10 * *ratio, HEIGHT + 9 * *ratio);
//         }
//         Event::KeyboardInput(action, _, Some(virt)) => {
//             let button = match virt {
//                 VirtualKeyCode::Z => Input::A,
//                 VirtualKeyCode::X => Input::B,
//                 VirtualKeyCode::Return => Input::Select,
//                 VirtualKeyCode::Comma => Input::Start,

//                 VirtualKeyCode::Left => Input::Left,
//                 VirtualKeyCode::Right => Input::Right,
//                 VirtualKeyCode::Down => Input::Down,
//                 VirtualKeyCode::Up => Input::Up,

//                 _ => return true,
//             };
//             match action {
//                 ElementState::Pressed => gb.keydown(button),
//                 ElementState::Released => gb.keyup(button),
//             }
//         }
//         _ => ()
//     }
//     return true
// }

// Shader sources
const VERTEX: &'static str = r"#version 150 core
in vec2 position;
in vec3 color;
in vec2 texcoord;
out vec3 Color;
out vec2 Texcoord;
void main() {
   Color = color;
   Texcoord = texcoord;
   gl_Position = vec4(position, 0.0, 1.0);
}
";

const FRAGMENT: &'static str = r"#version 150 core
in vec3 Color;
in vec2 Texcoord;
out vec4 outColor;
uniform sampler2D tex;
void main() {
   outColor = texture(tex, Texcoord);
}
";

// impl Glcx {
//     fn new(gl: gl::Gl) -> Glcx {
//         unsafe {
//             let mut vao = 0;
//             gl::GenVertexArrays(1, &mut vao);
//             gl::BindVertexArray(vao);

//             let mut vbo = 0;
//             gl::GenBuffers(1, &mut vbo);

//             const VERTICES: &'static [f32] = &[
//             //  Position   Color             Texcoords
//                 -1.0,  1.0, 1.0, 0.0, 0.0, 0.0, 0.0, // Top-left
//                  1.0,  1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // Top-right
//                  1.0, -1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // Bottom-right
//                 -1.0, -1.0, 1.0, 1.0, 1.0, 0.0, 1.0  // Bottom-left
//             ];
//             gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//             gl::BufferData(gl::ARRAY_BUFFER,
//                            (VERTICES.len() * 4) as libc::ssize_t,
//                            VERTICES.as_ptr() as *const _,
//                            gl::STATIC_DRAW);

//             let mut ebo = 0;
//             gl::GenBuffers(1, &mut ebo);

//             const ELEMENTS: &'static [GLuint] = &[
//                 0, 1, 2,
//                 2, 3, 0
//             ];

//             gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
//             gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
//                         (ELEMENTS.len() * mem::size_of::<GLuint>()) as libc::ssize_t,
//                         ELEMENTS.as_ptr() as *const _,
//                         gl::STATIC_DRAW);

//             // Create and compile the vertex shader
//             let vert = gl::CreateShader(gl::VERTEX_SHADER);
//             let src = CString::new(VERTEX).unwrap();
//             gl::ShaderSource(vert, 1, &src.as_ptr(), 0 as *const i32);
//             gl::CompileShader(vert);
//             Glcx::check_shader_compile(&gl, vert);

//             // Create and compile the fragment shader
//             let frag = gl::CreateShader(gl::FRAGMENT_SHADER);
//             let src = CString::new(FRAGMENT).unwrap();
//             gl::ShaderSource(frag, 1, &src.as_ptr(), 0 as *const i32);
//             gl::CompileShader(frag);
//             Glcx::check_shader_compile(&gl, frag);

//             // Link the vertex and fragment shader into a shader program
//             let program = gl::CreateProgram();
//             gl::AttachShader(program, vert);
//             gl::AttachShader(program, frag);
//             let buf = CString::new("outColor").unwrap();
//             gl::BindFragDataLocation(program, 0, buf.as_ptr());
//             gl::LinkProgram(program);
//             Glcx::check_program_link(&gl, program);
//             assert_eq!(gl::GetError(), 0);
//             gl::UseProgram(program);

//             // Specify the layout of the vertex data
//             let buf = CString::new("position").unwrap();
//             let pos_attrib = gl::GetAttribLocation(program, buf.as_ptr());
//             gl::EnableVertexAttribArray(pos_attrib as u32);
//             gl::VertexAttribPointer(pos_attrib as u32, 2, gl::FLOAT, gl::FALSE,
//                         (7 * mem::size_of::<GLfloat>()) as i32,
//                         0 as *const _);

//             let buf = CString::new("color").unwrap();
//             let col_attrib = gl::GetAttribLocation(program, buf.as_ptr());
//             gl::EnableVertexAttribArray(col_attrib as u32);
//             gl::VertexAttribPointer(col_attrib as u32, 3, gl::FLOAT, gl::FALSE,
//                         (7 * mem::size_of::<GLfloat>()) as i32,
//                         (2 * mem::size_of::<GLfloat>()) as *const _);

//             let buf = CString::new("texcoord").unwrap();
//             let tex_attrib = gl::GetAttribLocation(program, buf.as_ptr());
//             gl::EnableVertexAttribArray(tex_attrib as u32);
//             gl::VertexAttribPointer(tex_attrib as u32, 2, gl::FLOAT, gl::FALSE,
//                         (7 * mem::size_of::<GLfloat>()) as i32,
//                         (5 * mem::size_of::<GLfloat>()) as *const _);

//             // Load textures
//             let mut tex = 0;
//             gl::GenTextures(1, &mut tex);

//             gl::ActiveTexture(gl::TEXTURE0);
//             gl::BindTexture(gl::TEXTURE_2D, tex);
//             let buf = CString::new("tex").unwrap();
//             gl::Uniform1i(gl::GetUniformLocation(program, buf.as_ptr()), 0);

//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S,
//                              gl::CLAMP_TO_EDGE as i32);
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T,
//                              gl::CLAMP_TO_EDGE as i32);
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,
//                              gl::LINEAR as i32);
//             gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER,
//                              gl::LINEAR as i32);

//             Glcx {
//                 gl: gl,
//                 tex: tex,
//                 program: program,
//                 frag: frag,
//                 vert: vert,
//                 ebo: ebo,
//                 vbo: vbo,
//                 vao: vao,
//             }
//         }
//     }

//     unsafe fn check_shader_compile(gl: &gl::gl, shader: GLuint) {
//         let mut status = gl::FALSE as GLint;
//         gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
//         if status == (gl::TRUE as GLint) { return }

//         let mut len: GLint = 0;
//         gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
//         let mut buf = repeat(0u8).take(len as usize).collect::<Vec<_>>();
//         gl::GetShaderInfoLog(shader, len, ptr::null_mut(),
//                             buf.as_mut_ptr() as *mut GLchar);
//         panic!("{}", str::from_utf8(&buf).unwrap());
//     }

//     unsafe fn check_program_link(gl: &Glcx, program: GLuint) {
//         let mut status = gl::FALSE as GLint;
//         gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
//         if status == (gl::TRUE as GLint) { return }

//         let mut len: GLint = 0;
//         gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
//         let mut buf = repeat(0u8).take(len as usize).collect::<Vec<_>>();
//         gl::GetProgramInfoLog(program, len, ptr::null_mut(),
//                             buf.as_mut_ptr() as *mut GLchar);
//         panic!("{}", str::from_utf8(&buf).unwrap());
//     }

//     fn draw(&self, data: &[u8]) {
//         unsafe {
//             self.gl::ClearColor(0.0, 0.0, 1.0, 1.0);
//             self.gl::Clear(gl::COLOR_BUFFER_BIT);

//             self.gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32,
//                                WIDTH as i32, HEIGHT as i32,
//                                0, gl::RGBA, gl::UNSIGNED_BYTE,
//                                data.as_ptr() as *const _);
//             assert_eq!(self.gl::GetError(), 0);

//             // Draw a rectangle from the 2 triangles using 6
//             // indices
//             self.gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT,
//                                  0 as *const _);
//         }
//     }
// }

// impl Drop for Glcx {
//     fn drop(&mut self) {
//         unsafe {
//             self.gl::DeleteTextures(1, &self.tex);
//             self.gl::DeleteProgram(self.program);
//             self.gl::DeleteShader(self.vert);
//             self.gl::DeleteShader(self.frag);
//             self.gl::DeleteBuffers(1, &self.ebo);
//             self.gl::DeleteBuffers(1, &self.vbo);
//             self.gl::DeleteVertexArrays(1, &self.vao);
//         }
//     }
// }
