use crate::cpu::cpu::{Cpu, Memory};
use std::mem;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub struct Gb {
    cpu: Cpu,
    memory: Memory,
    fps: u32,
    cycles: u32,
}

impl Gb {
    pub fn new() -> Gb {
        let mut gb = Gb {
            memory: Memory::new(),
            cpu: Cpu::new(),
            fps: 0,
            cycles: 0,
        };
        gb.memory.power_on();
        return gb;
    }

    pub fn load(&mut self, rom: Vec<u8>) {
        self.memory.load_cartridge(rom);
    }

    // pub fn frame(&mut self) {
    //     // http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings
    //     // for the timing for this constant
    //     self.cycles += 70224;

    //     while self.cycles <= 70224 {
    //         let time = self.cpu.exec(&mut self.mem);
    //         self.mem.timer.step(time, &mut self.mem.if_, self.mem.speed);
    //         self.mem.gpu.step(time, &mut self.mem.if_);
    //         self.cycles -= time;
    //     }
    //     self.fps += 1;
    // }

    pub fn image(&self) -> &[u8] {
        &*self.memory.gpu.image_data
    }

    pub fn frames(&mut self) -> u32 {
        memory::replace(&mut self.fps, 0)
    }

    pub fn keydown(&mut self, key: input::Button) {
        self.memory.input.keydown(key, &mut self.memory.if_);
    }

    pub fn keyup(&mut self, key: input::Button) {
        self.memory.input.keyup(key);
    }

    // #[cfg(test)]
    // pub fn test_done(&self) -> bool {
    //     !self.mem.sound_on && self.cpu.is_loopback(&self.mem)
    // }
}
