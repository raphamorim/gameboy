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
        Self::try_new(data, filepath).unwrap()
    }

    /// Like `new`, but reports unsupported cartridges and unreadable saves
    /// instead of panicking. With a `filepath`, battery-backed cartridges keep
    /// their RAM in a `.gbsave` file next to it.
    pub fn try_new(
        data: Vec<u8>,
        filepath: Option<std::path::PathBuf>,
    ) -> Result<Gameboy, &'static str> {
        Ok(Gameboy {
            cpu: Cpu::try_new(data, filepath)?,
            width: 160,
            height: 144,
        })
    }

    /// Writes battery-backed cartridge RAM to its save file now.
    pub fn save(&self) -> Result<(), &'static str> {
        self.cpu.memory.mbc.save()
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
        // Initialize audio
        #[cfg(not(target_arch = "wasm32"))]
        let _audio_stream = match crate::sound::cpal_audio::CpalPlayer::new() {
            Some((player, stream)) => {
                self.enable_audio(Box::new(player));
                #[cfg(debug_assertions)]
                eprintln!("Audio enabled successfully");
                Some(stream)
            }
            None => {
                #[cfg(debug_assertions)]
                eprintln!("Failed to initialize audio");
                None
            }
        };
        #[cfg(target_arch = "wasm32")]
        let _audio_stream = None;

        if let Err(e) = crate::screen::desktop::run(self) {
            eprintln!("{e}");
        }
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
