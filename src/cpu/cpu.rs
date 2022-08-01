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

    fn ldrr_bb(c: &mut Cpu) { c._r.b=c._r.b; c._r.m=1; c._r.t=4; }
    fn ldrr_bc(c: &mut Cpu) { c._r.b=c._r.c; c._r.m=1; c._r.t=4; }
    fn ldrr_bd(c: &mut Cpu) { c._r.b=c._r.d; c._r.m=1; c._r.t=4; }
    fn ldrr_be(c: &mut Cpu) { c._r.b=c._r.e; c._r.m=1; c._r.t=4; }
    fn ldrr_bh(c: &mut Cpu) { c._r.b=c._r.h; c._r.m=1; c._r.t=4; }
    fn ldrr_bl(c: &mut Cpu) { c._r.b=c._r.l; c._r.m=1; c._r.t=4; }
    fn ldrr_ba(c: &mut Cpu) { c._r.b=c._r.a; c._r.m=1; c._r.t=4; }
    fn ldrr_cb(c: &mut Cpu) { c._r.c=c._r.b; c._r.m=1; c._r.t=4; }
    fn ldrr_cc(c: &mut Cpu) { c._r.c=c._r.c; c._r.m=1; c._r.t=4; }
    fn ldrr_cd(c: &mut Cpu) { c._r.c=c._r.d; c._r.m=1; c._r.t=4; }
    fn ldrr_ce(c: &mut Cpu) { c._r.c=c._r.e; c._r.m=1; c._r.t=4; }
    fn ldrr_ch(c: &mut Cpu) { c._r.c=c._r.h; c._r.m=1; c._r.t=4; }
    fn ldrr_cl(c: &mut Cpu) { c._r.c=c._r.l; c._r.m=1; c._r.t=4; }
    fn ldrr_ca(c: &mut Cpu) { c._r.c=c._r.a; c._r.m=1; c._r.t=4; }
    fn ldrr_db(c: &mut Cpu) { c._r.d=c._r.b; c._r.m=1; c._r.t=4; }
    fn ldrr_dc(c: &mut Cpu) { c._r.d=c._r.c; c._r.m=1; c._r.t=4; }
    fn ldrr_dd(c: &mut Cpu) { c._r.d=c._r.d; c._r.m=1; c._r.t=4; }
    fn ldrr_de(c: &mut Cpu) { c._r.d=c._r.e; c._r.m=1; c._r.t=4; }
    fn ldrr_dh(c: &mut Cpu) { c._r.d=c._r.h; c._r.m=1; c._r.t=4; }
    fn ldrr_dl(c: &mut Cpu) { c._r.d=c._r.l; c._r.m=1; c._r.t=4; }
    fn ldrr_da(c: &mut Cpu) { c._r.d=c._r.a; c._r.m=1; c._r.t=4; }
    fn ldrr_eb(c: &mut Cpu) { c._r.e=c._r.b; c._r.m=1; c._r.t=4; }
    fn ldrr_ec(c: &mut Cpu) { c._r.e=c._r.c; c._r.m=1; c._r.t=4; }
    fn ldrr_ed(c: &mut Cpu) { c._r.e=c._r.d; c._r.m=1; c._r.t=4; }
    fn ldrr_ee(c: &mut Cpu) { c._r.e=c._r.e; c._r.m=1; c._r.t=4; }
    fn ldrr_eh(c: &mut Cpu) { c._r.e=c._r.h; c._r.m=1; c._r.t=4; }
    fn ldrr_el(c: &mut Cpu) { c._r.e=c._r.l; c._r.m=1; c._r.t=4; }
    fn ldrr_ea(c: &mut Cpu) { c._r.e=c._r.a; c._r.m=1; c._r.t=4; }
    fn ldrr_hb(c: &mut Cpu) { c._r.h=c._r.b; c._r.m=1; c._r.t=4; }
    fn ldrr_hc(c: &mut Cpu) { c._r.h=c._r.c; c._r.m=1; c._r.t=4; }
    fn ldrr_hd(c: &mut Cpu) { c._r.h=c._r.d; c._r.m=1; c._r.t=4; }
    fn ldrr_he(c: &mut Cpu) { c._r.h=c._r.e; c._r.m=1; c._r.t=4; }
    fn ldrr_hh(c: &mut Cpu) { c._r.h=c._r.h; c._r.m=1; c._r.t=4; }
    fn ldrr_hl(c: &mut Cpu) { c._r.h=c._r.l; c._r.m=1; c._r.t=4; }
    fn ldrr_ha(c: &mut Cpu) { c._r.h=c._r.a; c._r.m=1; c._r.t=4; }
    fn ldrr_lb(c: &mut Cpu) { c._r.l=c._r.b; c._r.m=1; c._r.t=4; }
    fn ldrr_lc(c: &mut Cpu) { c._r.l=c._r.c; c._r.m=1; c._r.t=4; }
    fn ldrr_ld(c: &mut Cpu) { c._r.l=c._r.d; c._r.m=1; c._r.t=4; }
    fn ldrr_le(c: &mut Cpu) { c._r.l=c._r.e; c._r.m=1; c._r.t=4; }
    fn ldrr_lh(c: &mut Cpu) { c._r.l=c._r.h; c._r.m=1; c._r.t=4; }
    fn ldrr_ll(c: &mut Cpu) { c._r.l=c._r.l; c._r.m=1; c._r.t=4; }
    fn ldrr_la(c: &mut Cpu) { c._r.l=c._r.a; c._r.m=1; c._r.t=4; }
    fn ldrr_ab(c: &mut Cpu) { c._r.a=c._r.b; c._r.m=1; c._r.t=4; }
    fn ldrr_ac(c: &mut Cpu) { c._r.a=c._r.c; c._r.m=1; c._r.t=4; }
    fn ldrr_ad(c: &mut Cpu) { c._r.a=c._r.d; c._r.m=1; c._r.t=4; }
    fn ldrr_ae(c: &mut Cpu) { c._r.a=c._r.e; c._r.m=1; c._r.t=4; }
    fn ldrr_ah(c: &mut Cpu) { c._r.a=c._r.h; c._r.m=1; c._r.t=4; }
    fn ldrr_al(c: &mut Cpu) { c._r.a=c._r.l; c._r.m=1; c._r.t=4; }
    fn ldrr_aa(c: &mut Cpu) { c._r.a=c._r.a; c._r.m=1; c._r.t=4; }

    fn ldr_hlm_b(c: &mut Cpu, m: &mut Mmu) { c._r.b=m.rr8b((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
    fn ldr_hlm_c(c: &mut Cpu, m: &mut Mmu) { c._r.c=m.rr8b((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
    fn ldr_hlm_d(c: &mut Cpu, m: &mut Mmu) { c._r.d=m.rr8b((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
    fn ldr_hlm_e(c: &mut Cpu, m: &mut Mmu) { c._r.e=m.rr8b((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
    fn ldr_hlm_h(c: &mut Cpu, m: &mut Mmu) { c._r.h=m.rr8b((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
    fn ldr_hlm_l(c: &mut Cpu, m: &mut Mmu) { c._r.l=m.rr8b((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }
    fn ldr_hlm_a(c: &mut Cpu, m: &mut Mmu) { c._r.a=m.rr8b((c._r.h<<8)+c._r.l); c._r.m=2; c._r.t=8; }

    fn ld_hlmr_b(c: &mut Cpu, m: &mut Mmu) { m.ww8b((c._r.h<<8)+c._r.l,c._r.b); c._r.m=2; c._r.t=8; }
    fn ld_hlmr_c(c: &mut Cpu, m: &mut Mmu) { m.ww8b((c._r.h<<8)+c._r.l,c._r.c); c._r.m=2; c._r.t=8; }
    fn ld_hlmr_d(c: &mut Cpu, m: &mut Mmu) { m.ww8b((c._r.h<<8)+c._r.l,c._r.d); c._r.m=2; c._r.t=8; }
    fn ld_hlmr_e(c: &mut Cpu, m: &mut Mmu) { m.ww8b((c._r.h<<8)+c._r.l,c._r.e); c._r.m=2; c._r.t=8; }
    fn ld_hlmr_h(c: &mut Cpu, m: &mut Mmu) { m.ww8b((c._r.h<<8)+c._r.l,c._r.h); c._r.m=2; c._r.t=8; }
    fn ld_hlmr_l(c: &mut Cpu, m: &mut Mmu) { m.ww8b((c._r.h<<8)+c._r.l,c._r.l); c._r.m=2; c._r.t=8; }
    fn ld_hlmr_a(c: &mut Cpu, m: &mut Mmu) { m.ww8b((c._r.h<<8)+c._r.l,c._r.a); c._r.m=2; c._r.t=8; }

    fn ldrn_b(c: &mut Cpu, m: &mut Mmu) { c._r.b=m.r8b(c._r.pc); c._r.pc += 1; c._r.m=2; c._r.t=8; }
    fn ldrn_c(c: &mut Cpu, m: &mut Mmu) { c._r.c=m.r8b(c._r.pc); c._r.pc += 1; c._r.m=2; c._r.t=8; }
    fn ldrn_d(c: &mut Cpu, m: &mut Mmu) { c._r.d=m.r8b(c._r.pc); c._r.pc += 1; c._r.m=2; c._r.t=8; }
    fn ldrn_e(c: &mut Cpu, m: &mut Mmu) { c._r.e=m.r8b(c._r.pc); c._r.pc+=1; c._r.m=2; c._r.t=8; }
    fn ldrn_h(c: &mut Cpu, m: &mut Mmu) { c._r.h=m.r8b(c._r.pc); c._r.pc+=1; c._r.m=2; c._r.t=8; }
    fn ldrn_l(c: &mut Cpu, m: &mut Mmu) { c._r.l=m.r8b(c._r.pc); c._r.pc+=1; c._r.m=2; c._r.t=8; }
    fn ldrn_a(c: &mut Cpu, m: &mut Mmu) { c._r.a=m.r8b(c._r.pc); c._r.pc+=1; c._r.m=2; c._r.t=8; }

    // fn ldHLmn(c: &mut Cpu, m: &mut Mmu) { m.ww8b((c._r.h<<8)+c._r.l, m.r8b(c._r.pc)); c._r.pc+=1; c._r.m=3; c._r.t=12; }

    // fn ldBCmA(c: &mut Cpu, m: &mut Mmu) { m.ww8b((c._r.b<<8)+c._r.c, c._r.a); c._r.m=2; c._r.t=8; }
    // fn ldDEmA(c: &mut Cpu, m: &mut Mmu) { m.ww8b((c._r.d<<8)+c._r.e, c._r.a); c._r.m=2; c._r.t=8; }

    // fn ldmmA(c: &mut Cpu, m: &mut Mmu) { m.w8b(m.r16b(c._r.pc), c._r.a); c._r.pc+=2; c._r.m=4; c._r.t=16; }

    // fn ldABCm(c: &mut Cpu, m: &mut Mmu) { c._r.a=m.rr8b((c._r.b<<8)+c._r.c); c._r.m=2; c._r.t=8; }
    // fn ldADEm(c: &mut Cpu, m: &mut Mmu) { c._r.a=m.rr8b((c._r.d<<8)+c._r.e); c._r.m=2; c._r.t=8; }

    // fn ldAmm(c: &mut Cpu, m: &mut Mmu) { c._r.a=m.r8b(m.r16b(c._r.pc)); c._r.pc+=2; c._r.m=4; c._r.t=16; }

    // fn ldBCnn(c: &mut Cpu, m: &mut Mmu) { c._r.c=m.r8b(c._r.pc); c._r.b=m.r8b(c._r.pc+1); c._r.pc+=2; c._r.m=3; c._r.t=12; }
    // fn ldDEnn(c: &mut Cpu, m: &mut Mmu) { c._r.e=m.r8b(c._r.pc); c._r.d=m.r8b(c._r.pc+1); c._r.pc+=2; c._r.m=3; c._r.t=12; }
    // fn ldHLnn(c: &mut Cpu, m: &mut Mmu) { c._r.l=m.r8b(c._r.pc); c._r.h=m.r8b(c._r.pc+1); c._r.pc+=2; c._r.m=3; c._r.t=12; }
    // fn ldSPnn(c: &mut Cpu, m: &mut Mmu) { c._r.sp=m.r16b(c._r.pc); c._r.pc+=2; c._r.m=3; c._r.t=12; }

    // fn ldHLmm(c: &mut Cpu, m: &mut Mmu) { let i=m.r16b(c._r.pc); c._r.pc+=2; c._r.l=m.r8b(i); c._r.h=m.r8b(i+1); c._r.m=5; c._r.t=20; }
    // fn ldmmHL(c: &mut Cpu, m: &mut Mmu) { let i=m.r16b(c._r.pc); c._r.pc+=2; m.w16b(i,(c._r.h<<8)+c._r.l); c._r.m=5; c._r.t=20; }

    // fn ldHLIA(c: &mut Cpu, m: &mut Mmu) { m.w8b((c._r.h<<8)+c._r.l, c._r.a); c._r.l=(c._r.l+1)&255; if(!c._r.l) c._r.h=(c._r.h+1)&255; c._r.m=2; c._r.t=8; }
    // fn ldAHLI(c: &mut Cpu, m: &mut Mmu) { c._r.a=m.r8b((c._r.h<<8)+c._r.l); c._r.l=(c._r.l+1)&255; if(!c._r.l) c._r.h=(c._r.h+1)&255; c._r.m=2; c._r.t=8; }

    // fn ldHldA(c: &mut Cpu, m: &mut Mmu) { m.w8b((c._r.h<<8)+c._r.l, c._r.a); c._r.l=(c._r.l-1)&255; if(c._r.l==255) c._r.h=(c._r.h-1)&255; c._r.m=2; c._r.t=8; }
    // fn ldAHld(c: &mut Cpu, m: &mut Mmu) { c._r.a=m.r8b((c._r.h<<8)+c._r.l); c._r.l=(c._r.l-1)&255; if(c._r.l==255) c._r.h=(c._r.h-1)&255; c._r.m=2; c._r.t=8; }

    // fn ldAIOn(c: &mut Cpu, m: &mut Mmu) { c._r.a=m.r8b(0xFF00+m.r8b(c._r.pc)); c._r.pc++; c._r.m=3; c._r.t=12; }
    // fn ldIOnA(c: &mut Cpu, m: &mut Mmu) { m.w8b(0xFF00+m.r8b(c._r.pc),c._r.a); c._r.pc++; c._r.m=3; c._r.t=12; }
    // fn ldAIOC(c: &mut Cpu, m: &mut Mmu) { c._r.a=m.r8b(0xFF00+c._r.c); c._r.m=2; c._r.t=8; }
    // fn ldIOCA(c: &mut Cpu, m: &mut Mmu) { m.w8b(0xFF00+c._r.c,c._r.a); c._r.m=2; c._r.t=8; }

    // fn ldHLSPn(c: &mut Cpu, m: &mut Mmu) { let i=m.r8b(c._r.pc); if(i>127) i=-((~i+1)&255); c._r.pc++; i+=c._r.sp; c._r.h=(i>>8)&255; c._r.l=i&255; c._r.m=3; c._r.t=12; }

    // fn SWAPr_b(c: &mut Cpu, m: &mut Mmu) { let tr=c._r.b; c._r.b=m.r8b((c._r.h<<8)+c._r.l); m.w8b((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
    // fn SWAPr_c(c: &mut Cpu, m: &mut Mmu) { let tr=c._r.c; c._r.c=m.r8b((c._r.h<<8)+c._r.l); m.w8b((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
    // fn SWAPr_d(c: &mut Cpu, m: &mut Mmu) { let tr=c._r.d; c._r.d=m.r8b((c._r.h<<8)+c._r.l); m.w8b((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
    // fn SWAPr_e(c: &mut Cpu, m: &mut Mmu) { let tr=c._r.e; c._r.e=m.r8b((c._r.h<<8)+c._r.l); m.w8b((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
    // fn SWAPr_h(c: &mut Cpu, m: &mut Mmu) { let tr=c._r.h; c._r.h=m.r8b((c._r.h<<8)+c._r.l); m.w8b((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
    // fn SWAPr_l(c: &mut Cpu, m: &mut Mmu) { let tr=c._r.l; c._r.l=m.r8b((c._r.h<<8)+c._r.l); m.w8b((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }
    // fn SWAPr_a(c: &mut Cpu, m: &mut Mmu) { let tr=c._r.a; c._r.a=m.r8b((c._r.h<<8)+c._r.l); m.w8b((c._r.h<<8)+c._r.l,tr); c._r.m=4; c._r.t=16; }

    pub fn exec(&mut self, m: &mut Mmu) {
        'outer: loop {
            self._r.r = (self._r.r + 1) & 127;
            let counter = m.r8b(self._r.pc);
            self._r.pc += 1;
            println!("{:?}", counter);
            if counter == 255 {
                break 'outer;
            }
            self.program_counter_call(counter, m);
            self._r.pc &= 65535;
            // Z80._map[op]();
            self.clock.m += self._r.m;                  // Add time to CPU clock
            self.clock.t += self._r.t;       
        }
    }

    fn nop(&mut self) { self._r.m = 1; self._r.t = 4; }
    fn program_counter_call(&mut self, op: u8, m: &mut Mmu) {
        match op {
            0 => self.nop(),
            1 => self.ld_bcnn(m),
            2_u8..=u8::MAX => self.nop(),
        }
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

    fn ld_bcnn(&mut self, m: &mut Mmu) { 
        self._r.c=m.r8b(self._r.pc);
        self._r.b=m.r8b(self._r.pc+1);
        self._r.pc+=2;
        self._r.m=3;
        self._r.t=12; 
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

    // Read a byte from absolute location into A (ld A, addr)
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
