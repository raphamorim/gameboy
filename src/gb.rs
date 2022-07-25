use crate::cpu::cpu::{Memory, Cpu};
use std::memory;

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

    pub fn frame(&mut self) {
        // http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings
        self.cycles += 70224;

        while self.cycles <= 70224 {
            let time = self.cpu.exec(&mut self.memory);
            self.memory.timer.step(time, &mut self.memory.if_, self.memory.speed);
            self.memory.gpu.step(time, &mut self.memory.if_);
            self.cycles -= time;
        }

        self.fps += 1;
    }

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

    #[cfg(test)]
    pub fn test_done(&self) -> bool {
        !self.memory.sound_on && self.cpu.is_loopback(&self.memory)
    }
}