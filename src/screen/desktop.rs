extern crate gl;
extern crate glutin;
extern crate libc;

use crate::gameboy::Gameboy;
use crate::input::KeypadKey;

use std::error::Error;
use std::ffi::CString;
use std::mem;
use std::num::NonZeroU32;
use std::ptr;
use std::str;
use std::time::{Duration, Instant};

use gl::types::*;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext,
    Version,
};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, Surface, SwapInterval, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

pub struct Glcx {
    #[allow(unused)]
    tex: GLuint,
    #[allow(unused)]
    program: GLuint,
    #[allow(unused)]
    frag: GLuint,
    #[allow(unused)]
    vert: GLuint,
    #[allow(unused)]
    ebo: GLuint,
    #[allow(unused)]
    vbo: GLuint,
    #[allow(unused)]
    vao: GLuint,
}

const FRAME_DURATION: Duration = Duration::from_millis(16); // ~60 FPS

/// Opens a window and runs `gameboy` in it until the window is closed.
pub fn run(gameboy: Gameboy) -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    let mut app = DesktopApp {
        gameboy,
        gl: None,
        focused: true,
        next_frame: Instant::now(),
    };
    event_loop.run_app(&mut app)?;
    Ok(())
}

struct GlState {
    window: Window,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    cx: Glcx,
}

struct DesktopApp {
    gameboy: Gameboy,
    gl: Option<GlState>,
    focused: bool,
    next_frame: Instant,
}

impl DesktopApp {
    fn create_gl(&self, event_loop: &ActiveEventLoop) -> Result<GlState, Box<dyn Error>> {
        let attributes = Window::default_attributes()
            .with_title("Gameboy")
            .with_inner_size(LogicalSize::new(self.gameboy.width, self.gameboy.height))
            .with_resizable(true);
        let (window, config) = DisplayBuilder::new()
            .with_window_attributes(Some(attributes))
            .build(event_loop, ConfigTemplateBuilder::new(), |mut configs| {
                configs.next().expect("no OpenGL config available")
            })?;
        let window = window.ok_or("could not create a window")?;
        let display = config.display();

        // The shaders are GLSL 150, which needs a 3.2 core context.
        let raw_handle = window.window_handle().ok().map(|h| h.as_raw());
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 2))))
            .build(raw_handle);
        let context = unsafe { display.create_context(&config, &context_attributes)? };
        let surface_attributes = window.build_surface_attributes(Default::default())?;
        let surface =
            unsafe { display.create_window_surface(&config, &surface_attributes)? };
        let context = context.make_current(&surface)?;

        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            display.get_proc_address(&symbol) as *const _
        });
        let _ = surface
            .set_swap_interval(&context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()));

        Ok(GlState {
            window,
            surface,
            context,
            cx: Glcx::new(),
        })
    }
}

impl ApplicationHandler for DesktopApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gl.is_some() {
            return;
        }
        match self.create_gl(event_loop) {
            Ok(gl) => self.gl = Some(gl),
            Err(e) => {
                eprintln!("Failed to create the OpenGL window: {e}");
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Focused(focused) => self.focused = focused,
            WindowEvent::Resized(size) => {
                if let (Some(gl), Some(width), Some(height)) = (
                    self.gl.as_ref(),
                    NonZeroU32::new(size.width),
                    NonZeroU32::new(size.height),
                ) {
                    gl.surface.resize(&gl.context, width, height);
                    unsafe { gl::Viewport(0, 0, size.width as i32, size.height as i32) };
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => {
                let button = match code {
                    KeyCode::KeyA => KeypadKey::A,
                    KeyCode::KeyB => KeypadKey::B,
                    KeyCode::KeyZ => KeypadKey::Select,
                    KeyCode::KeyX => KeypadKey::Start,

                    KeyCode::ArrowLeft => KeypadKey::Left,
                    KeyCode::ArrowRight => KeypadKey::Right,
                    KeyCode::ArrowDown => KeypadKey::Down,
                    KeyCode::ArrowUp => KeypadKey::Up,

                    _ => return,
                };
                match state {
                    ElementState::Pressed => self.gameboy.keydown(button),
                    ElementState::Released => self.gameboy.keyup(button),
                }
            }
            WindowEvent::RedrawRequested => {
                if let (true, Some(gl)) = (self.focused, self.gl.as_ref()) {
                    self.gameboy.frame();
                    gl.cx.draw(
                        self.gameboy.width,
                        self.gameboy.height,
                        self.gameboy.image(),
                    );
                    let _ = gl.surface.swap_buffers(&gl.context);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        if now >= self.next_frame {
            if let Some(gl) = self.gl.as_ref() {
                gl.window.request_redraw();
            }
            self.next_frame = now + FRAME_DURATION;
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(self.next_frame));
    }
}

// Shader sources
const VERTEX: &str = r"#version 150 core
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

const FRAGMENT: &str = r"#version 150 core
in vec3 Color;
in vec2 Texcoord;
out vec4 outColor;
uniform sampler2D tex;
void main() {
   outColor = texture(tex, Texcoord);
}
";

impl Glcx {
    pub fn new() -> Glcx {
        unsafe {
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);

            const VERTICES: &[f32] = &[
                //  Position   Color             Texcoords
                -1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, // Top-left
                1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // Top-right
                1.0, -1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // Bottom-right
                -1.0, -1.0, 1.0, 1.0, 1.0, 0.0, 1.0, // Bottom-left
            ];
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTICES.len() * 4) as libc::ssize_t,
                VERTICES.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);

            const ELEMENTS: &[GLuint] = &[0, 1, 2, 2, 3, 0];

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(ELEMENTS) as libc::ssize_t,
                ELEMENTS.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let vert = gl::CreateShader(gl::VERTEX_SHADER);
            let src = CString::new(VERTEX).unwrap();
            gl::ShaderSource(vert, 1, &src.as_ptr(), std::ptr::null::<i32>());
            gl::CompileShader(vert);
            // Glcx::check_shader_compile(&gl, vert);

            // Create and compile the fragment shader
            let frag = gl::CreateShader(gl::FRAGMENT_SHADER);
            let src = CString::new(FRAGMENT).unwrap();
            gl::ShaderSource(frag, 1, &src.as_ptr(), std::ptr::null::<i32>());
            gl::CompileShader(frag);
            // TODO: check_shader_compile(frag)

            let program = gl::CreateProgram();
            gl::AttachShader(program, vert);
            gl::AttachShader(program, frag);
            let buf = CString::new("outColor").unwrap();
            gl::BindFragDataLocation(program, 0, buf.as_ptr());
            gl::LinkProgram(program);
            // TODO: check_program_link(program)
            assert_eq!(gl::GetError(), 0);
            gl::UseProgram(program);

            let buf = CString::new("position").unwrap();
            let pos_attrib = gl::GetAttribLocation(program, buf.as_ptr());
            gl::EnableVertexAttribArray(pos_attrib as u32);
            gl::VertexAttribPointer(
                pos_attrib as u32,
                2,
                gl::FLOAT,
                gl::FALSE,
                (7 * mem::size_of::<GLfloat>()) as i32,
                std::ptr::null(),
            );

            let buf = CString::new("color").unwrap();
            let col_attrib = gl::GetAttribLocation(program, buf.as_ptr());
            gl::EnableVertexAttribArray(col_attrib as u32);
            gl::VertexAttribPointer(
                col_attrib as u32,
                3,
                gl::FLOAT,
                gl::FALSE,
                (7 * mem::size_of::<GLfloat>()) as i32,
                (2 * mem::size_of::<GLfloat>()) as *const _,
            );

            let buf = CString::new("texcoord").unwrap();
            let tex_attrib = gl::GetAttribLocation(program, buf.as_ptr());
            gl::EnableVertexAttribArray(tex_attrib as u32);
            gl::VertexAttribPointer(
                tex_attrib as u32,
                2,
                gl::FLOAT,
                gl::FALSE,
                (7 * mem::size_of::<GLfloat>()) as i32,
                (5 * mem::size_of::<GLfloat>()) as *const _,
            );

            // Load textures
            let mut tex = 0;
            gl::GenTextures(1, &mut tex);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            let buf = CString::new("tex").unwrap();
            gl::Uniform1i(gl::GetUniformLocation(program, buf.as_ptr()), 0);

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            Glcx {
                tex,
                program,
                frag,
                vert,
                ebo,
                vbo,
                vao,
            }
        }
    }

    #[allow(dead_code)]
    unsafe fn check_program_link(_gl: &Glcx, program: GLuint) {
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status == (gl::TRUE as GLint) {
            return;
        }

        let mut len: GLint = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = vec![0u8; len as usize];
        gl::GetProgramInfoLog(
            program,
            len,
            ptr::null_mut(),
            buf.as_mut_ptr() as *mut GLchar,
        );
        panic!("{}", str::from_utf8(&buf).unwrap());
    }

    pub fn draw(&self, width: u32, height: u32, data: &[u8]) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
            assert_eq!(gl::GetError(), 0);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}
