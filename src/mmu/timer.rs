use crate::cpu::cpu::Interrupt;
use crate::mmu::mmu::Speed;

#[derive(Debug)]
pub struct Timer {
    clock: Clock,

    // See http://nocash.emubase.de/pandocs.htm#timeranddividerregisters
    pub div: u8,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,

    tima_speed: u32,
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
            tima_speed: 256,
            clock: Clock { tima: 0, div: 0 },
        }
    }

    pub fn update(&mut self) {
        // See step() function for timings
        match self.tac & 0x3 {
            0x0 => {
                self.tima_speed = 256;
            }
            0x1 => {
                self.tima_speed = 4;
            }
            0x2 => {
                self.tima_speed = 16;
            }
            0x3 => {
                self.tima_speed = 64;
            }
            _ => {}
        }
    }

    // Details: http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-Timers
    pub fn step(&mut self, ticks: u32, if_: &mut u8, speed: Speed) {
        // undo the multiplication in the cpu
        let ticks = match speed {
            Speed::Normal => ticks / 4,
            Speed::Double => ticks,
        };
        self.clock.div += ticks;

        // CPU runs on a 4,194,304 Hz clock, although the argument to this
        // function runs at 1/4 that rate, so 1,048,576 Hz. The div register is
        // a clock that runs at 16384 Hz, which is 1/64 the 1 MHz clock.
        //
        // The TAC register then controls the timer rate of the TIMA register.
        // The value is controlled by 3 bits in TAC:
        //
        //      Bit 2    - Timer Stop  (0=Stop, 1=Start)
        //      Bits 1-0 - Input Clock Select
        //                 00:   4096 Hz = 1/256 of 1 MHz
        //                 01: 262144 Hz = 1/4   of 1 MHz
        //                 10:  65536 Hz = 1/16  of 1 MHz
        //                 11:  16384 Hz = 1/64  of 1 MHz
        //

        // Increment the DIV timer as necessary (1/64th the speed)
        while self.clock.div >= 64 {
            if self.div < u8::MAX {
                self.div += 1;
            }
            self.clock.div -= 64;
        }

        // Increment the TIMA timer as necessary (variable speed)
        if self.tac & 0x4 != 0 {
            self.clock.tima += ticks;
            while self.clock.tima >= self.tima_speed {
                self.tima += 1;
                if self.tima == 0 {
                    self.tima = self.tma;
                    *if_ |= Interrupt::Timer as u8;
                }
                self.clock.tima -= self.tima_speed;
            }
        }
    }
}
