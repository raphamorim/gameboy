use crate::mode::GbMode;
use crate::cpu::cpu::Cpu;
use crate::input::KeypadKey;
use std::sync::mpsc::{Receiver};

pub struct Gameboy {
    cpu: Cpu<'static>,
    pub width: u32,
    pub height: u32,
}

pub use self::Target::{GameBoy, GameBoyColor, SuperGameBoy};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum RenderMode {
    Desktop,
    WebAssembly,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Target {
    GameBoy,
    GameBoyColor,
    SuperGameBoy,
}

pub const CYCLES: u32 = 70224;

impl<'a> Gameboy {
    pub fn new(filepath: &str) -> Gameboy {
        let gb = Gameboy {
            cpu: Cpu::new(GbMode::Classic, filepath),
            width: 160,
            height: 144,
        };

        gb
    }

    // pub fn load_rom_with_u8_vec(&mut self, rom: Vec<u8>) {
    //     self.cpu.memory.load_rom(rom);
    // }

    // #[cfg(feature = "desktop")]
    // pub fn load_rom(&mut self, filepath: &str) -> Result<bool, String> {
    //     use std::fs::File;
    //     use std::io::Read;

    //     let mut rom = Vec::new();
    //     if filepath == "" {
    //         return Err(String::from("Please provide a valid filepath"));
    //     }

    //     let file = File::open(filepath);
    //     match file.and_then(|mut f| f.read_to_end(&mut rom)) {
    //         Ok(..) => {}
    //         Err(e) => return Err(format!("Failed to read {}: {}", filepath, e)),
    //     };

    //     self.cpu.load_rom(rom);

    //     Ok(true)
    // }

    pub fn render(self, render_mode: RenderMode) {
        match render_mode {
            RenderMode::Desktop => {
                // #[cfg(feature = "desktop")]
                self.render_desktop();
            }
            RenderMode::WebAssembly => {
                // crate::screen::web::render();
            }
        }
    }

    pub fn render_desktop(mut self) {
        use crate::screen::desktop::*;

        let event_loop: glutin::event_loop::EventLoop<()> =
            glutin::event_loop::EventLoop::with_user_event();
        let inner_size = glutin::dpi::LogicalSize {
            width: self.width,
            height: self.height,
        };
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("LR35902")
            .with_inner_size(inner_size)
            .with_resizable(true);
        let gl_window = glutin::ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .unwrap();
        let gl_window = unsafe { gl_window.make_current().unwrap() };

        gl::load_with(|s| gl_window.get_proc_address(s) as *const _);

        let cx = Glcx::new();
        let mut focused = true;
        event_loop.run(move |event, _, control_flow| {
            let window = gl_window.window();
            match event {
                glutin::event::Event::WindowEvent {
                    window_id: _,
                    event: wevent,
                } => *control_flow = process_window(window, &wevent, &mut self, &mut focused),
                glutin::event::Event::MainEventsCleared => window.request_redraw(),
                glutin::event::Event::RedrawRequested(_) => {
                    if focused == true {
                        self.frame();
                        cx.draw(self.width, self.height, self.image());
                        gl_window.swap_buffers().unwrap();
                    }

                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                _ => {
                    let next_frame_time =
                        std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
                    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
                }
            }
        });
    }

    pub fn check_and_reset_gpu_updated(&mut self) -> bool {
        let result = self.cpu.memory.gpu.updated;
        self.cpu.memory.gpu.updated = false;
        result
    }

    pub fn frame(&mut self) {
        // let waitticks = (4194304f64 / 1000.0 * 16.0).round() as u32;
        let waitticks = CYCLES;
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
}
