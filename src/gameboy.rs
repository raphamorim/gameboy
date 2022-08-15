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
pub enum Target {
    GameBoy,
    GameBoyColor,
    SuperGameBoy,
}

#[warn(dead_code)]
fn add_yellow_color(text: &str) -> String {
    format!("{}{}{}", "\x1b[93m", text, "\x1b[0m")
}

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

    pub fn load(&mut self, rom: Vec<u8>) {
        self.cpu.memory.load_rom(rom);
    }

    #[cfg(feature = "desktop")]
    pub fn read_rom_by_filepath(&mut self, filepath: &str) -> Vec<u8> {
        use std::fs::File;
        use std::io::Read;

        let cmd = add_yellow_color("[lr35902]");
        let mut rom = Vec::new();
        if filepath == "" {
            println!("{} Please provide a rom file", cmd);
            return rom;
        }

        println!("{} ROM Path: {:?}", cmd, filepath);
        let file = File::open(filepath);
        match file.and_then(|mut f| f.read_to_end(&mut rom)) {
            Ok(..) => {}
            Err(e) => {
                println!("failed to read {}: {}", filepath, e);
            }
        };

        rom
    }

    pub fn frame(&mut self) {
        self.cycles += 70224;

        // while self.cycles <= 70224 {
        while self.cycles <= 80000 {
            let time = self.cpu.exec();
            // println!("{:?} {:?}", self.cycles, time);
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
