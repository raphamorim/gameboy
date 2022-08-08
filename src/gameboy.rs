use crate::cpu::cpu::Cpu;
use crate::mmu::mmu::Mmu;

pub const WIDTH: u32 = 160;
pub const HEIGHT: u32 = 144;

pub struct Gameboy {
    cpu: Cpu,
    fps: u32,
    cycles: u32,
    // memory: Mmu,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        let memory = Mmu::new();
        let mut gb = Gameboy {
            cpu: Cpu::new(memory),
            fps: 0,
            cycles: 0,
        };

        gb.cpu.mmu.power_on();
        gb
    }

    pub fn load(&mut self, rom: Vec<u8>) {
        self.cpu.mmu.load_rom(rom);
    }

    pub fn reset() {
        // TODO: reset memory, cpu
    }

    pub fn frame(&mut self) {
        // self.cycles += 70224;

        // while self.cycles <= 70224 {
        //     let time = self.cpu.exec(&mut self.mem);
        //     self.memory.timer.step(time);
        //     self.memory.gpu.step(time, &mut self.memory.f_flag);
        //     self.cycles -= time;
        // }

        // self.fps += 1;
        self.cpu.exec();
    }

    pub fn image(&self) -> &[u8] {
        &*self.cpu.mmu.gpu.image_data
    }

    // pub fn frames(&mut self) -> u32 {
    //     memory::replace(&mut self.fps, 0)
    // }

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
