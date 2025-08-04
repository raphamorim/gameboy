use crate::cpu::core::Cpu;
use crate::input::KeypadKey;

pub struct Gameboy {
    cpu: Cpu<'static>,
    pub width: u32,
    pub height: u32,
}

pub use self::Target::{GameBoy, GameBoyColor, SuperGameBoy};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default)]
pub enum RenderMode {
    #[cfg(not(any(target_arch = "wasm32", feature = "ffi")))]
    #[default]
    Desktop,
    #[cfg(not(any(target_arch = "wasm32", feature = "ffi")))]
    Terminal,
    #[cfg(any(target_arch = "wasm32", feature = "ffi"))]
    #[default]
    WebAssembly,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Target {
    GameBoy,
    GameBoyColor,
    SuperGameBoy,
}

#[cfg(not(any(target_arch = "wasm32", feature = "ffi")))]
pub fn load_rom(filepath: &str) -> Result<(Vec<u8>, std::path::PathBuf), String> {
    use std::fs::File;
    use std::io::Read;

    let mut rom = Vec::new();
    if filepath.is_empty() {
        return Err(String::from("Please provide a valid filepath"));
    }

    let file = File::open(filepath);
    match file.and_then(|mut f| f.read_to_end(&mut rom)) {
        Ok(_) => {}
        Err(e) => return Err(format!("Failed to read {:?}: {}", filepath, e)),
    };

    Ok((rom, std::path::PathBuf::from(filepath)))
}

// Match rboy's timing: (4194304 / 1000 * 16) = ~67,109 cycles per 16ms frame
pub const CYCLES: u32 = 67109;

impl Gameboy {
    pub fn new(data: Vec<u8>, filepath: Option<std::path::PathBuf>) -> Gameboy {
        // let rom = load_rom();

        let gb = Gameboy {
            cpu: Cpu::new(data, filepath),
            width: 160,
            height: 144,
        };

        gb
    }

    pub fn render(self, render_mode: RenderMode) {
        match render_mode {
            #[cfg(not(any(target_arch = "wasm32", feature = "ffi")))]
            RenderMode::Desktop => {
                self.render_desktop();
            }
            #[cfg(any(target_arch = "wasm32", feature = "ffi"))]
            RenderMode::WebAssembly => {
                // crate::screen::web::render();
            }
            #[cfg(not(any(target_arch = "wasm32", feature = "ffi")))]
            RenderMode::Terminal => {
                self.render_terminal();
            }
        }
    }

    #[cfg(not(any(target_arch = "wasm32", feature = "ffi")))]
    pub fn render_desktop(mut self) {
        use crate::screen::desktop::*;

        // Initialize audio
        #[cfg(not(target_arch = "wasm32"))]
        let _audio_stream = match crate::sound::cpal_audio::CpalPlayer::new() {
            Some((player, stream)) => {
                self.enable_audio(Box::new(player));
                eprintln!("Audio enabled successfully");
                Some(stream)
            }
            None => {
                eprintln!("Failed to initialize audio");
                None
            }
        };
        #[cfg(target_arch = "wasm32")]
        let _audio_stream = None;

        let event_loop: glutin::event_loop::EventLoop<()> =
            glutin::event_loop::EventLoop::with_user_event();
        let inner_size = glutin::dpi::LogicalSize {
            width: self.width,
            height: self.height,
        };
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Gameboy")
            .with_inner_size(inner_size)
            .with_resizable(true);
        let gl_window = glutin::ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .unwrap();
        let gl_window = unsafe { gl_window.make_current().unwrap() };

        gl::load_with(|s| gl_window.get_proc_address(s) as *const _);

        let cx = Glcx::new();
        let mut focused = true;
        let mut last_frame_time = std::time::Instant::now();
        let frame_duration = std::time::Duration::from_millis(16); // ~60 FPS

        event_loop.run(move |event, _, control_flow| {
            let window = gl_window.window();
            match event {
                glutin::event::Event::WindowEvent {
                    window_id: _,
                    event: wevent,
                } => {
                    *control_flow =
                        process_window(window, &wevent, &mut self, &mut focused)
                }
                glutin::event::Event::MainEventsCleared => {
                    // Only request redraw if enough time has passed
                    let now = std::time::Instant::now();
                    if now.duration_since(last_frame_time) >= frame_duration {
                        window.request_redraw();
                    }
                }
                glutin::event::Event::RedrawRequested(_) => {
                    if focused {
                        self.frame();
                        cx.draw(self.width, self.height, self.image());
                        gl_window.swap_buffers().unwrap();
                        last_frame_time = std::time::Instant::now();
                    }

                    // Schedule next frame
                    let next_frame_time = last_frame_time + frame_duration;
                    *control_flow =
                        glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
                }
                _ => {
                    // Keep the current control flow
                }
            }
        });
    }

    #[cfg(not(any(target_arch = "wasm32", feature = "ffi")))]
    pub fn render_terminal(self) {
        use crate::screen::tui;

        let _ = tui::run(self);
    }

    #[inline]
    pub fn check_and_reset_gpu_updated(&mut self) -> bool {
        let result = self.cpu.memory.gpu.updated;
        self.cpu.memory.gpu.updated = false;
        result
    }

    pub fn frame(&mut self) {
        // Use rboy's exact timing calculation
        let waitticks = (4194304f64 / 1000.0 * 16.0).round() as u32;
        let mut ticks = 0;

        'outer: loop {
            while ticks < waitticks {
                ticks += self.cpu.do_cycle();
                if self.check_and_reset_gpu_updated() {
                    break 'outer;
                }
            }

            ticks -= waitticks;
        }
    }

    pub fn image(&self) -> &[u8] {
        &*self.cpu.memory.gpu.data
    }

    pub fn image_mut(&mut self) -> &mut [u8] {
        &mut *self.cpu.memory.gpu.data
    }

    pub fn keydown(&mut self, key: KeypadKey) {
        self.cpu.memory.keypad.keydown(key);
    }

    pub fn keyup(&mut self, key: KeypadKey) {
        self.cpu.memory.keypad.keyup(key);
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn enable_audio(&mut self, player: Box<dyn crate::sound::AudioPlayer>) {
        self.cpu.memory.enable_audio(player);
    }
}
