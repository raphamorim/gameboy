use crate::cpu::registers::{Clock, Registers};

#[derive(Debug)]
pub struct Cpu {
    _r: Registers, // registers
                   // clock: Clock
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            _r: Registers::default(),
        }
    }

    pub fn exec() {}

    // Add E to A, leaving result in A (ADD A, E)
    fn ADDr_e(&mut self) {
        // Perform addition
        self._r.a += self._r.e;
        // Clear flags
        self._r.f = 0;
        // Check for zero
        // !(Z80._r.a & 255)
        if self._r.a == 0 {
            self._r.f |= 0x80;
        }
        // Check for carry
        if self._r.a > 255 {
            self._r.f |= 0x10;
        }
        // Mask to 8-bits
        self._r.a &= 255;
        // 1 M-time taken
        self._r.m = 1;
        self._r.t = 4;
    }

    // Compare B to A, setting flags (CP A, B)
    fn CPr_b(&mut self) {
        // Temp copy of A
        let i = self._r.a;
        // Subtract B
        i -= self._r.b;
        // Set subtraction flag
        self._r.f |= 0x40;
        // Check for zero
        // (!(i & 255)) {
        if i == 0 {
            self._r.f |= 0x80;
        }
        // Check for underflow
        if i < 0 {
            self._r.f |= 0x10;
        }
        // 1 M-time taken
        self._r.m = 1;
        self._r.t = 4;
    }

    // No-operation (NOP)
    fn NOP(&mut self) {
        self._r.m = 1;
        // 1 M-time taken
        self._r.t = 4;
    }
}

pub struct Memory {}
