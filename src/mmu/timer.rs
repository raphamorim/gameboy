use crate::cpu::cpu::Interrupt;
use crate::mmu::mmu::Speed;

#[derive(Debug)]
pub struct Timer {
    clock: Clock,

    pub div: u8,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,

    step: u32,
}

#[derive(Debug)]
struct Clock {
    tima: u32,
    div: u32,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            step: 256,
            clock: Clock { tima: 0, div: 0 },
        }
    }

    pub fn update(&mut self) {
        // See step() function for timings
        match self.tac & 0x3 {
            0x0 => {
                self.step = 256;
            }
            0x1 => {
                self.step = 4;
            }
            0x2 => {
                self.step = 16;
            }
            0x3 => {
                self.step = 64;
            }
            _ => {}
        }
    }

    // Details: http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-Timers
    pub fn step(&mut self, ticks: u32, if_: &mut u8, speed: Speed) {
        // undo the multiplication in the cpu
        let ticks = match speed {
            Speed::Normal => {
                // ticks / 4
                ticks
            }
            Speed::Double => ticks,
        };

        self.clock.div += ticks;
        while self.clock.div >= 256 {
            self.div = self.div.wrapping_add(1);
            self.clock.div -= 256;
        }

        // Increment the TIMA timer as necessary (variable speed)
        if self.tac & 0x4 != 0 {
            self.clock.tima += ticks;

            while self.clock.tima >= self.step {
                self.tima = self.tima.wrapping_add(1);
                if self.tima == 0 {
                    self.tima = self.tma;
                    *if_ |= Interrupt::Timer as u8;
                }
                self.clock.tima -= self.step;
            }
        }
    }
}
