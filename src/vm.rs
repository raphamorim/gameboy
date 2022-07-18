use crate::cpu::Cpu;
use crate::gpu::GpuMode;
use log::{debug};

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub struct Vm {
    pub cpu: Cpu,
    pub buffer: Vec<u32>,
}

impl Vm {
    pub fn new(binary: Vec<u8>) -> Self {
        Self {
            cpu: Cpu::new(binary),
            buffer: vec![0; WIDTH * HEIGHT],
        }
    }

    pub fn run(&mut self) -> Result<(), ()> {
        // TODO: better way to control this
        while self.cpu.bus.gpu.mode != GpuMode::VBlank {
            self.cpu.step()?;
        }
        self.cpu.bus.gpu.build_screen(&mut self.buffer);
        while self.cpu.bus.gpu.mode == GpuMode::VBlank {
            self.cpu.step()?;
        }
        Ok(())
    }

    pub fn dump(&self) {
        debug!("{}", self.cpu.dump());
    }
}
