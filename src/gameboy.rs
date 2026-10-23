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

    /// Identifies the cartridge: the header and global checksums.
    fn cartridge_id(&self) -> [u8; 3] {
        let mbc = &self.cpu.memory.mbc;
        [mbc.readrom(0x14D), mbc.readrom(0x14E), mbc.readrom(0x14F)]
    }

    /// A snapshot of the whole machine, which `load_state` can restore.
    pub fn save_state(&self) -> Vec<u8> {
        use crate::state::{State, Writer, MAGIC, VERSION};
        let mut w = Writer::default();
        w.bytes(MAGIC);
        VERSION.save_state(&mut w);
        w.bytes(&self.cartridge_id());
        self.cpu.save_state(&mut w);
        w.0
    }

    /// Restores a snapshot from `save_state`. On failure the machine is left
    /// as it was.
    pub fn load_state(&mut self, data: &[u8]) -> Result<(), &'static str> {
        use crate::state::{Reader, State, MAGIC, VERSION, WRONG_GAME};
        let mut r = Reader::new(data);
        if r.bytes(MAGIC.len()).ok() != Some(MAGIC.as_slice()) {
            return Err("This file isn't a save state.");
        }
        let mut version = 0u16;
        version.load_state(&mut r)?;
        if version != VERSION {
            return Err("The save state is from an unsupported version.");
        }
        if r.bytes(3)? != self.cartridge_id() {
            return Err(WRONG_GAME);
        }
        // A failure can stop partway through, so keep a copy to roll back to.
        let backup = self.save_state();
        let result = self.cpu.load_state(&mut r);
        if result.is_err() {
            let mut r = Reader::new(&backup[MAGIC.len() + 2 + 3..]);
            let _ = self.cpu.load_state(&mut r);
        }
        result
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

#[cfg(test)]
mod tests {
    use super::Gameboy;

    /// A 32 KiB ROM-only cartridge that counts in A and stores it to WRAM
    /// forever, so the machine state changes every frame.
    fn rom(checksum: u8) -> Vec<u8> {
        let mut rom = vec![0u8; 0x8000];
        // 0x100: INC A; LD (0xC000), A; JR back to 0x100
        rom[0x100..0x106].copy_from_slice(&[0x3C, 0xEA, 0x00, 0xC0, 0x18, 0xFA]);
        rom[0x14E] = checksum;
        rom
    }

    #[test]
    fn restored_state_replays_identically() {
        let mut gb = Gameboy::new(rom(1), None);
        (0..10).for_each(|_| gb.frame());
        let state = gb.save_state();

        (0..30).for_each(|_| gb.frame());
        let expected = (gb.image().to_vec(), gb.save_state());

        gb.load_state(&state).unwrap();
        assert_eq!(gb.save_state(), state);
        (0..30).for_each(|_| gb.frame());
        assert_eq!((gb.image().to_vec(), gb.save_state()), expected);
    }

    #[test]
    fn rejects_other_games_and_damaged_states() {
        let mut gb = Gameboy::new(rom(1), None);
        (0..5).for_each(|_| gb.frame());
        let before = gb.save_state();

        let other = Gameboy::new(rom(2), None).save_state();
        assert!(gb.load_state(&other).is_err());
        assert!(gb.load_state(&before[..before.len() / 2]).is_err());
        assert!(gb.load_state(b"not a state").is_err());
        // Every rejected load left the running game untouched.
        assert_eq!(gb.save_state(), before);
    }
}
