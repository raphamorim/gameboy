use crate::cpu::cpu::Cpu;
use crate::input::Button;
use crate::mmu::mmu::Mmu;

pub struct Gameboy {
    cpu: Cpu,
    cycles: u32,
    scale: u8,
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

impl Gameboy {
    pub fn new() -> Gameboy {
        let memory: Mmu = Mmu::new(Target::GameBoy);
        let mut gb = Gameboy {
            cpu: Cpu::new(memory),
            cycles: 0,
            scale: 1,
            width: 160,
            height: 144,
        };

        gb.cpu.memory.power_on();
        gb
    }

    pub fn load_rom_with_u8_vec(&mut self, rom: Vec<u8>) {
        self.cpu.memory.load_rom(rom);
    }

    #[cfg(feature = "desktop")]
    pub fn load_rom(&mut self, filepath: &str) -> Result<bool, String> {
        use std::fs::File;
        use std::io::Read;

        let mut rom = Vec::new();
        if filepath == "" {
            return Err(String::from("Please provide a valid filepath"))
        }

        let file = File::open(filepath);
        match file.and_then(|mut f| f.read_to_end(&mut rom)) {
            Ok(..) => {}
            Err(e) => {
                return Err(format!("Failed to read {}: {}", filepath, e))
            }
        };

        self.cpu.memory.load_rom(rom);

        // TODO: return Self to use function render
        Ok(true)
    }

    pub fn render(self, render_mode: RenderMode) {
        match render_mode {
            RenderMode::Desktop => {
                #[cfg(feature = "desktop")]
                crate::screen::desktop::render(self);
            },
            RenderMode::WebAssembly => {
                // crate::screen::web::render();
            }
        }
    }

    pub fn frame(&mut self) {
        self.cycles += CYCLES;

        // Runs two CPU cycles per frame
        while self.cycles <= CYCLES * 2 {
            let time = self.cpu.exec();
            self.cpu
                .memory
                .timer
                .step(time, &mut self.cpu.memory.if_, self.cpu.memory.speed);
            self.cpu.memory.gpu.step(time, &mut self.cpu.memory.if_);
            if time > self.cycles {
                break;
            } else {
                self.cycles -= time;
            }
        }
    }

    pub fn image(&self) -> &[u8] {
        &*self.cpu.memory.gpu.image_data
    }

    pub fn image_mut(&mut self) -> &mut [u8] {
        &mut *self.cpu.memory.gpu.image_data
    }

    pub fn set_scale(&mut self, scale: u8) -> &mut Gameboy {
        self.scale = scale;
        self.width *= scale as u32;
        self.height *= scale as u32;
        self
    }

    pub fn keydown(&mut self, key: Button) {
        self.cpu.memory.input.keydown(key, &mut self.cpu.memory.if_);
    }

    pub fn keyup(&mut self, key: Button) {
        self.cpu.memory.input.keyup(key);
    }
}
