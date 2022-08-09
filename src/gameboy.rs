use crate::cpu::cpu::Cpu;
use crate::mmu::mmu::Mmu;

pub const WIDTH: u32 = 160;
pub const HEIGHT: u32 = 144;

pub struct Gameboy {
    cpu: Cpu,
    cycles: u32,
    // memory: Mmu,
}

pub use self::Target::{GameBoy, GameBoyColor, SuperGameBoy};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Target {
    GameBoy,
    GameBoyColor,
    SuperGameBoy,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        let memory = Mmu::new(Target::GameBoy);
        let mut gb = Gameboy {
            cpu: Cpu::new(memory),
            cycles: 0,
        };

        gb.cpu.memory.power_on();
        gb
    }

    pub fn load(&mut self, rom: Vec<u8>) {
        self.cpu.memory.load_rom(rom);
    }

    pub fn reset() {
        // TODO: reset memory, cpu
    }

    pub fn frame(&mut self) {
        self.cycles += 70224;

        while self.cycles <= 70224 {
            let time = self.cpu.exec();
            println!("{:?} {:?}", self.cycles, time);
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

    // pub fn keydown(&mut self, key: input::Button) {
    //     self.memory.input.keydown(key, &mut self.memory.if_);
    // }

    // pub fn keyup(&mut self, key: input::Button) {
    //     self.memory.input.keyup(key);
    // }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
// fn test_create_gb() {
// let country: Country = get_countries("SE").ok().unwrap();
// assert_eq!(country.language_code, "sv");
// }
// }
