use crate::cpu::registers::{Clock, Registers};
use crate::mmu::mmu::Mmu;

#[derive(Debug)]
pub struct Cpu {
    _r: Registers, // registers
    clock: Clock,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            _r: Registers::default(),
            clock: Clock { m: 0, t: 0 },
        }
    }

    fn load_instructions() {
        // self.instructions = vec![
        //     self.no_operation,
        //     self.LDBCnn,
        //     self.LDBCmA,
        //     self.INCBC,
        //     self.INCr_b,
        // ];
    }

    // cpu.exec(&mut self.mem)
    pub fn exec(&mut self, m: &mut Mmu) -> u32 {
        let op = m.r8b(self._r.pc);
        self._r.pc += 1;
        // Z80._map[op]();
        self._r.pc.into()
    }

    // Add E to A, leaving result in A (ADD A, E)
    fn add_register_e(&mut self) {
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
    fn compare_register_b(&mut self) {
        // Temp copy of A
        let mut i = self._r.a;
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

    fn no_operation(&mut self) {
        self._r.m = 1;
        // 1 M-time taken
        self._r.t = 4;
    }

    // Push registers B and C to the stack (PUSH BC)
    fn push_registers_b_c(&mut self, m: &mut Mmu) {
        // Drop through the stack
        self._r.sp -= 1;
        // Write B
        Mmu::w8b(m, self._r.sp, self._r.b);
        // Drop through the stack
        self._r.sp -= 1;
        // Write C
        Mmu::w8b(m, self._r.sp, self._r.c);
        // 3 M-times taken
        self._r.m = 3;
        self._r.t = 12;
    }

    // Pop registers H and L off the stack (POP HL)
    fn pop_registers_h_l(&mut self, m: &mut Mmu) {
        // Read L
        self._r.l = Mmu::r8b(m, self._r.sp);
        // Move back up the stack
        self._r.sp += 1;
        // Read H
        self._r.h = Mmu::r8b(m, self._r.sp);
        // Move back up the stack
        self._r.sp += 1;
        // 3 M-times taken
        self._r.m = 3;
        self._r.t = 12;
    }

    // Read a byte from absolute location into A (LD A, addr)
    fn ldamm(&mut self, m: &mut Mmu) {
        // Get address from instr
        let addr = Mmu::r16b(m, self._r.pc);
        // Advance Program Counter
        self._r.pc += 2;
        // Read from address
        self._r.a = Mmu::r8b(m, addr);
        // 4 M-times taken
        self._r.m = 4;
        self._r.t = 16;
    }
}
