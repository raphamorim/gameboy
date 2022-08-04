use crate::cpu::data;
use crate::cpu::registers::{Clock, Registers};
use crate::mmu::mmu::Mmu;

#[derive(Debug)]
pub struct Cpu {
    pub _r: Registers, // registers
    pub clock: Clock,
    _halt: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            _r: Registers::default(),
            _halt: 0,
            clock: Clock { m: 0, t: 0 },
        }
    }

    fn nop(&mut self) {
        self._r.m = 1;
        self._r.t = 4;
    }
    fn halt(&mut self) {
        self._halt = 1;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn di(&mut self) {
        self._r.ime = 0;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ei(&mut self) {
        self._r.ime = 1;
        self._r.m = 1;
        self._r.t = 4;
    }
    pub fn fz(&mut self, i: u8, cond: u8) {
        self._r.f = 0;
        if i == 0 || i > 255 {
            self._r.f |= 128;
        }
        if cond > 0 {
            self._r.f |= 0x40;
        } else {
            self._r.f |= 0;
        }
    }

    fn xx(&mut self) {
        /*Undefined map entry*/
        let opc = self._r.pc - 1;
        println!("Instruction at {} no implemented, stopping.", opc);
        // self._stop=1;
    }

    fn ldrr_bb(&mut self) {
        self._r.b = self._r.b;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_bc(&mut self) {
        self._r.b = self._r.c;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_bd(&mut self) {
        self._r.b = self._r.d;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_be(&mut self) {
        self._r.b = self._r.e;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_bh(&mut self) {
        self._r.b = self._r.h;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_bl(&mut self) {
        self._r.b = self._r.l;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ba(&mut self) {
        self._r.b = self._r.a;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_cb(&mut self) {
        self._r.c = self._r.b;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_cc(&mut self) {
        self._r.c = self._r.c;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_cd(&mut self) {
        self._r.c = self._r.d;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ce(&mut self) {
        self._r.c = self._r.e;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ch(&mut self) {
        self._r.c = self._r.h;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_cl(&mut self) {
        self._r.c = self._r.l;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ca(&mut self) {
        self._r.c = self._r.a;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_db(&mut self) {
        self._r.d = self._r.b;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_dc(&mut self) {
        self._r.d = self._r.c;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_dd(&mut self) {
        self._r.d = self._r.d;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_de(&mut self) {
        self._r.d = self._r.e;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_dh(&mut self) {
        self._r.d = self._r.h;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_dl(&mut self) {
        self._r.d = self._r.l;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_da(&mut self) {
        self._r.d = self._r.a;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_eb(&mut self) {
        self._r.e = self._r.b;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ec(&mut self) {
        self._r.e = self._r.c;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ed(&mut self) {
        self._r.e = self._r.d;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ee(&mut self) {
        self._r.e = self._r.e;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_eh(&mut self) {
        self._r.e = self._r.h;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_el(&mut self) {
        self._r.e = self._r.l;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ea(&mut self) {
        self._r.e = self._r.a;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_hb(&mut self) {
        self._r.h = self._r.b;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_hc(&mut self) {
        self._r.h = self._r.c;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_hd(&mut self) {
        self._r.h = self._r.d;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_he(&mut self) {
        self._r.h = self._r.e;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_hh(&mut self) {
        self._r.h = self._r.h;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_hl(&mut self) {
        self._r.h = self._r.l;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ha(&mut self) {
        self._r.h = self._r.a;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_lb(&mut self) {
        self._r.l = self._r.b;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_lc(&mut self) {
        self._r.l = self._r.c;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ld(&mut self) {
        self._r.l = self._r.d;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_le(&mut self) {
        self._r.l = self._r.e;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_lh(&mut self) {
        self._r.l = self._r.h;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ll(&mut self) {
        self._r.l = self._r.l;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_la(&mut self) {
        self._r.l = self._r.a;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ab(&mut self) {
        self._r.a = self._r.b;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ac(&mut self) {
        self._r.a = self._r.c;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ad(&mut self) {
        self._r.a = self._r.d;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ae(&mut self) {
        self._r.a = self._r.e;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_ah(&mut self) {
        self._r.a = self._r.h;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_al(&mut self) {
        self._r.a = self._r.l;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_aa(&mut self) {
        self._r.a = self._r.a;
        self._r.m = 1;
        self._r.t = 4;
    }

    fn ldr_hlm_b(&mut self, m: &mut Mmu) {
        self._r.b = m.rr8b((self._r.h << 8) + self._r.l);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_c(&mut self, m: &mut Mmu) {
        self._r.c = m.rr8b((self._r.h << 8) + self._r.l);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_d(&mut self, m: &mut Mmu) {
        self._r.d = m.rr8b((self._r.h << 8) + self._r.l);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_e(&mut self, m: &mut Mmu) {
        self._r.e = m.rr8b((self._r.h << 8) + self._r.l);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_h(&mut self, m: &mut Mmu) {
        self._r.h = m.rr8b((self._r.h << 8) + self._r.l);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_l(&mut self, m: &mut Mmu) {
        self._r.l = m.rr8b((self._r.h << 8) + self._r.l);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_a(&mut self, m: &mut Mmu) {
        self._r.a = m.rr8b((self._r.h << 8) + self._r.l);
        self._r.m = 2;
        self._r.t = 8;
    }

    fn ld_hlmr_b(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.h << 8) + self._r.l, self._r.b);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_c(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.h << 8) + self._r.l, self._r.c);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_d(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.h << 8) + self._r.l, self._r.d);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_e(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.h << 8) + self._r.l, self._r.e);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_h(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.h << 8) + self._r.l, self._r.h);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_l(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.h << 8) + self._r.l, self._r.l);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_a(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.h << 8) + self._r.l, self._r.a);
        self._r.m = 2;
        self._r.t = 8;
    }

    fn ldrn_b(&mut self, m: &mut Mmu) {
        self._r.b = m.r8b(self._r.pc);
        self._r.pc += 1;
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_c(&mut self, m: &mut Mmu) {
        self._r.c = m.r8b(self._r.pc);
        self._r.pc += 1;
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_d(&mut self, m: &mut Mmu) {
        self._r.d = m.r8b(self._r.pc);
        self._r.pc += 1;
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_e(&mut self, m: &mut Mmu) {
        self._r.e = m.r8b(self._r.pc);
        self._r.pc += 1;
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_h(&mut self, m: &mut Mmu) {
        self._r.h = m.r8b(self._r.pc);
        self._r.pc += 1;
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_l(&mut self, m: &mut Mmu) {
        self._r.l = m.r8b(self._r.pc);
        self._r.pc += 1;
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_a(&mut self, m: &mut Mmu) {
        self._r.a = m.r8b(self._r.pc);
        self._r.pc += 1;
        self._r.m = 2;
        self._r.t = 8;
    }

    fn ld_hlmn(&mut self, m: &mut Mmu) {
        let addr = m.r8b(self._r.pc);
        m.ww8b((self._r.h << 8) + self._r.l, addr);
        self._r.pc += 1;
        self._r.m = 3;
        self._r.t = 12;
    }

    fn ld_bcm_a(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.b << 8) + self._r.c, self._r.a);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_dem_a(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.d << 8) + self._r.e, self._r.a);
        self._r.m = 2;
        self._r.t = 8;
    }

    fn ldmm_a(&mut self, m: &mut Mmu) {
        let addr = m.r16b(self._r.pc);
        m.w8b(addr, self._r.a);
        self._r.pc += 2;
        self._r.m = 4;
        self._r.t = 16;
    }

    fn ld_abcm(&mut self, m: &mut Mmu) {
        self._r.a = m.rr8b((self._r.b << 8) + self._r.c);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_adem(&mut self, m: &mut Mmu) {
        self._r.a = m.rr8b((self._r.d << 8) + self._r.e);
        self._r.m = 2;
        self._r.t = 8;
    }

    fn ld_amm(&mut self, m: &mut Mmu) {
        let addr = m.r16b(self._r.pc);
        self._r.a = m.r8b(addr);
        self._r.pc += 2;
        self._r.m = 4;
        self._r.t = 16;
    }

    fn ld_bcnn(&mut self, m: &mut Mmu) {
        self._r.c = m.r8b(self._r.pc);
        self._r.b = m.r8b(self._r.pc + 1);
        self._r.pc += 2;
        self._r.m = 3;
        self._r.t = 12;
    }
    fn ld_denn(&mut self, m: &mut Mmu) {
        self._r.e = m.r8b(self._r.pc);
        self._r.d = m.r8b(self._r.pc + 1);
        self._r.pc += 2;
        self._r.m = 3;
        self._r.t = 12;
    }
    fn ld_hlnn(&mut self, m: &mut Mmu) {
        self._r.l = m.r8b(self._r.pc);
        self._r.h = m.r8b(self._r.pc + 1);
        self._r.pc += 2;
        self._r.m = 3;
        self._r.t = 12;
    }
    fn ld_spnn(&mut self, m: &mut Mmu) {
        self._r.sp = m.r16b(self._r.pc);
        self._r.pc += 2;
        self._r.m = 3;
        self._r.t = 12;
    }

    fn ld_hlmm(&mut self, m: &mut Mmu) {
        let i = m.r16b(self._r.pc);
        self._r.pc += 2;
        self._r.l = m.r8b(i);
        self._r.h = m.r8b(i + 1);
        self._r.m = 5;
        self._r.t = 20;
    }
    fn ldmm_hl(&mut self, m: &mut Mmu) {
        let addr = m.r16b(self._r.pc);
        self._r.pc += 2;
        let value = (self._r.h << 8) + self._r.l;
        m.ww16b(addr, value);
        self._r.m = 5;
        self._r.t = 20;
    }

    fn ld_hlia(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.h << 8) + self._r.l, self._r.a);
        self._r.l = (self._r.l + 1) & 255;
        if self._r.l == 0 {
            self._r.h = (self._r.h + 1) & 255;
        }
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_ahli(&mut self, m: &mut Mmu) {
        self._r.a = m.rr8b((self._r.h << 8) + self._r.l);
        self._r.l = (self._r.l + 1) & 255;
        if self._r.l == 0 {
            self._r.h = (self._r.h + 1) & 255;
        }
        self._r.m = 2;
        self._r.t = 8;
    }

    fn ld_hld_a(&mut self, m: &mut Mmu) {
        m.ww8b((self._r.h << 8) + self._r.l, self._r.a);
        self._r.l = (self._r.l - 1) & 255;
        if self._r.l == 255 {
            self._r.h = (self._r.h - 1) & 255;
        }
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_ahld(&mut self, m: &mut Mmu) {
        self._r.a = m.rr8b((self._r.h << 8) + self._r.l);
        self._r.l = (self._r.l - 1) & 255;
        if self._r.l == 255 {
            self._r.h = (self._r.h - 1) & 255;
        }
        self._r.m = 2;
        self._r.t = 8;
    }

    fn ld_aion(&mut self, m: &mut Mmu) {
        let addr = 0xFF00 + m.r16b(self._r.pc);
        self._r.a = m.r8b(addr);
        self._r.pc += 1;
        self._r.m = 3;
        self._r.t = 12;
    }
    fn ld_ion_a(&mut self, m: &mut Mmu) {
        let addr = 0xFF00 + m.r16b(self._r.pc);
        m.w8b(addr, self._r.a);
        self._r.pc += 1;
        self._r.m = 3;
        self._r.t = 12;
    }
    fn ld_aioc(&mut self, m: &mut Mmu) {
        let addr: u16 = (0xFF00 + self._r.c as u16).into();
        self._r.a = m.r8b(addr);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_ioca(&mut self, m: &mut Mmu) {
        let addr: u16 = (0xFF00 + self._r.c as u16).into();
        m.w8b(addr, self._r.a);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlspn(&mut self, m: &mut Mmu) {
        let i = m.r8b(self._r.pc);
        if i > 127 {
            // i=-((!i+1)&255);
        }
        self._r.pc += 1;
        // i += self._r.sp.into();
        self._r.h = (i >> 8) & 255;
        self._r.l = i & 255;
        self._r.m = 3;
        self._r.t = 12;
    }

    fn swapr_b(&mut self, m: &mut Mmu) {
        let tr = self._r.b;
        let addr = ((self._r.h << 8) + self._r.l) as u16;
        self._r.b = m.r8b(addr);
        let waddr = ((self._r.h << 8) + self._r.l) as u16;
        m.w8b(waddr, tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_c(&mut self, m: &mut Mmu) {
        let tr = self._r.c;
        let addr = ((self._r.h << 8) + self._r.l) as u16;
        self._r.c = m.r8b(addr);
        let waddr = ((self._r.h << 8) + self._r.l) as u16;
        m.w8b(waddr, tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_d(&mut self, m: &mut Mmu) {
        let tr = self._r.d;
        self._r.d = m.r8b(((self._r.h << 8) + self._r.l).into());
        m.w8b(((self._r.h << 8) + self._r.l).into(), tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_e(&mut self, m: &mut Mmu) {
        let tr = self._r.e;
        self._r.e = m.r8b(((self._r.h << 8) + self._r.l).into());
        m.w8b(((self._r.h << 8) + self._r.l).into(), tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_h(&mut self, m: &mut Mmu) {
        let tr = self._r.h;
        self._r.h = m.r8b(((self._r.h << 8) + self._r.l).into());
        m.w8b(((self._r.h << 8) + self._r.l).into(), tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_l(&mut self, m: &mut Mmu) {
        let tr = self._r.l;
        self._r.l = m.r8b(((self._r.h << 8) + self._r.l).into());
        m.w8b(((self._r.h << 8) + self._r.l).into(), tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_a(&mut self, m: &mut Mmu) {
        let tr = self._r.a;
        self._r.a = m.r8b(((self._r.h << 8) + self._r.l).into());
        m.w8b(((self._r.h << 8) + self._r.l).into(), tr);
        self._r.m = 4;
        self._r.t = 16;
    }

    pub fn exec(&mut self, m: &mut Mmu) {
        'outer: loop {
            self._r.r = (self._r.r + 1) & 127;
            let counter = m.r8b(self._r.pc);
            println!("{:?}", counter);
            if counter == 255 {
                break 'outer;
            }
            self.program_counter_call(counter, m);
            self._r.pc += 1;
            self._r.pc &= 65535;
            self.clock.m += self._r.m; // Add time to CPU clock
            self.clock.t += self._r.t;
        }
    }

    fn program_counter_call(&mut self, op: u8, m: &mut Mmu) {
        match op {
            0 => self.nop(),
            1 => self.ld_bcnn(m),
            1 => self.ld_bcm_a(m),
            2 => data::incbc(self),
            3 => data::incr_b(self),
            4 => data::decr_b(self),
            5 => self.ldrn_b(m),
            // 6 => self.rlca(m),
            // 7 => self.ldmmsp(m),
            8 => data::addhlbc(self),
            9 => self.ld_abcm(m),
            10 => data::decbc(self),
            11 => data::incr_c(self),
            12 => data::decr_c(self),
            13 => self.ldrn_c(m),
            // 14 => self.rrca(m),
            // 15 => self.djnzn(m),
            16 => self.ld_denn(m),
            17 => self.ld_dem_a(m),
            18 => data::incde(self),
            19 => data::incr_d(self),
            20 => data::decr_d(self),
            21 => self.ldrn_d(m),
            // 22 => self.rla(m),
            // 23 => self.jrn(m),
            24 => data::addhlde(self),
            25 => self.ld_adem(m),
            26 => data::decde(self),
            27 => data::incr_e(self),
            28 => data::decr_e(self),
            29 => self.ldrn_e(m),
            // 30 => self.rra(m),
            // 31 => self.jrnzn(m),
            32 => self.ld_hlnn(m),
            33 => self.ld_hlia(m),
            34 => data::inchl(self),
            35 => data::incr_h(self),
            36 => data::decr_h(self),
            37 => self.ldrn_h(m),
            38 => self.xx(),
            // 39 => self.jrzn(m),
            40 => data::addhlhl(self),
            41 => self.ld_ahli(m),
            42 => data::dechl(self),
            43 => data::incr_l(self),
            44 => data::decr_l(self),
            45 => self.ldrn_l(m),
            // 46 => self.cpl(m),
            // 47 => self.jrncn(m),
            48 => self.ld_spnn(m),
            49 => self.ld_hld_a(m),
            50 => data::incsp(self),
            51 => data::inchlm(self, m),
            52 => data::dechlm(self, m),
            53 => self.ld_hlmn(m),
            // 54 => self.scf(m),
            // 55 => self.jrcn(m),
            56 => data::addhlsp(self),
            57 => self.ld_ahld(m),
            58 => data::decsp(self),
            59 => data::incr_a(self),
            60 => data::decr_a(self),
            61 => self.ldrn_a(m),
            // 62 => self.ccf(m),
            63 => self.ldrr_bb(),
            64 => self.ldrr_bc(),
            65 => self.ldrr_bd(),
            66 => self.ldrr_be(),
            67 => self.ldrr_bh(),
            68 => self.ldrr_bl(),
            69 => self.ldr_hlm_b(m),
            70 => self.ldrr_ba(),
            71 => self.ldrr_cb(),
            72 => self.ldrr_cc(),
            73 => self.ldrr_cd(),
            74 => self.ldrr_ce(),
            75 => self.ldrr_ch(),
            76 => self.ldrr_cl(),
            77 => self.ldr_hlm_c(m),
            78 => self.ldrr_ca(),
            79 => self.ldrr_db(),
            80 => self.ldrr_dc(),
            81 => self.ldrr_dd(),
            82 => self.ldrr_de(),
            83 => self.ldrr_dh(),
            84 => self.ldrr_dl(),
            85 => self.ldr_hlm_d(m),
            86 => self.ldrr_da(),
            87 => self.ldrr_eb(),
            88 => self.ldrr_ec(),
            89 => self.ldrr_ed(),
            90 => self.ldrr_ee(),
            91 => self.ldrr_eh(),
            92 => self.ldrr_el(),
            93 => self.ldr_hlm_e(m),
            94 => self.ldrr_ea(),
            95 => self.ldrr_hb(),
            96 => self.ldrr_hc(),
            97 => self.ldrr_hd(),
            98 => self.ldrr_he(),
            99 => self.ldrr_hh(),
            100 => self.ldrr_hl(),
            101 => self.ldr_hlm_h(m),
            102 => self.ldrr_ha(),
            103 => self.ldrr_lb(),
            104 => self.ldrr_lc(),
            105 => self.ldrr_ld(),
            106 => self.ldrr_le(),
            107 => self.ldrr_lh(),
            108 => self.ldrr_ll(),
            109 => self.ldr_hlm_l(m),
            110 => self.ldrr_la(),
            111 => self.ld_hlmr_b(m),
            112 => self.ld_hlmr_c(m),
            113 => self.ld_hlmr_d(m),
            114 => self.ld_hlmr_e(m),
            115 => self.ld_hlmr_h(m),
            116 => self.ld_hlmr_l(m),
            117 => self.halt(),
            118 => self.ld_hlmr_a(m),
            119 => self.ldrr_ab(),
            120 => self.ldrr_ac(),
            121 => self.ldrr_ad(),
            122 => self.ldrr_ae(),
            123 => self.ldrr_ah(),
            124 => self.ldrr_al(),
            125 => self.ldr_hlm_a(m),
            126 => self.ldrr_aa(),
            127 => data::addr_b(self),
            128 => data::addr_c(self),
            129 => data::addr_d(self),
            130 => data::addr_e(self),
            131 => data::addr_h(self),
            132 => data::addr_l(self),
            133 => data::addhl(self, m),
            134 => data::addr_a(self),
            135 => data::adcr_b(self),
            136 => data::adcr_c(self),
            137 => data::adcr_d(self),
            138 => data::adcr_e(self),
            139 => data::adcr_h(self),
            140 => data::adcr_l(self),
            141 => data::adchl(self, m),
            142 => data::adcr_a(self),
            143 => data::subr_b(self),
            144 => data::subr_c(self),
            145 => data::subr_d(self),
            146 => data::subr_e(self),
            147 => data::subr_h(self),
            148 => data::subr_l(self),
            149 => data::subhl(self, m),
            150 => data::subr_a(self),
            151 => data::sbcr_b(self),
            152 => data::sbcr_c(self),
            153 => data::sbcr_d(self),
            154 => data::sbcr_e(self),
            155 => data::sbcr_h(self),
            156 => data::sbcr_l(self),
            157 => data::sbchl(self, m),
            158 => data::sbcr_a(self),
            159 => data::andr_b(self),
            160 => data::andr_c(self),
            161 => data::andr_d(self),
            162 => data::andr_e(self),
            163 => data::andr_h(self),
            164 => data::andr_l(self),
            165 => data::andhl(self, m),
            166 => data::andr_a(self),
            167 => data::xorr_b(self),
            168 => data::xorr_c(self),
            169 => data::xorr_d(self),
            170 => data::xorr_e(self),
            171 => data::xorr_h(self),
            172 => data::xorr_l(self),
            173 => data::xorhl(self, m),
            174 => data::xorr_a(self),
            175 => data::orr_b(self),
            176 => data::orr_c(self),
            177 => data::orr_d(self),
            178 => data::orr_e(self),
            179 => data::orr_h(self),
            180 => data::orr_l(self),
            181 => data::orhl(self, m),
            182 => data::orr_a(self),
            183 => data::cpr_b(self),
            184 => data::cpr_c(self),
            185 => data::cpr_d(self),
            186 => data::cpr_e(self),
            187 => data::cpr_h(self),
            188 => data::cpr_l(self),
            189 => data::cphl(self, m),
            190 => data::cpr_a(self),
            //           191 => self.retnz(m),
            //           192 => self.popbc(m),
            //           193 => self.jpnznn(m),
            //           194 => self.jpnn(m),
            //           195 => self.callnznn(m),
            //           196 => self.pushbc(m),
            //           197 => self.addn(m),
            //           198 => self.rst00(m),
            //           199 => self.retz(m),
            //           200 => self.ret(m),
            //           201 => self.jpznn(m),
            //           202 => self.mapcb(m),
            //           203 => self.callznn(m),
            //           204 => self.callnn(m),
            //           205 => self.adcn(m),
            //           206 => self.rst08(m),

            // // D0
            //           207 => self.retnc(m),
            //           208 => self.popde(m),
            //           209 => self.jpncnn(m),
            //           210 => self.xx(m),
            //           211 => self.callncnn(m),
            //           212 => self.pushde(m),
            //           213 => self.subn(m),
            //           214 => self.rst10(m),
            //           215 => self.retc(m),
            //           216 => self.reti(m),
            //           217 => self.jpcnn(m),
            //           218 => self.xx(m),
            //           219 => self.callcnn(m),
            //           220 => self.xx(m),
            //           221 => self.sbcn(m),
            //           222 => self.rst18(m),

            // // E0
            //           223 => self.ldiona(m),
            //           224 => self.pophl(m),
            //           225 => self.ldioca(m),
            //           226 => self.xx(m),
            //           227 => self.xx(m),
            //           228 => self.pushhl(m),
            //           229 => self.andn(m),
            //           230 => self.rst20(m),
            //           231 => self.addspn(m),
            //           232 => self.jphl(m),
            //           233 => self.ldmma(m),
            //           234 => self.xx(m),
            //           235 => self.xx(m),
            //           236 => self.xx(m),
            //           237 => self.orn(m),
            //           238 => self.rst28(m),

            //           // F0
            //           239 => self.ldaion(m),
            //           240 => self.popaf(m),
            //           241 => self.ldaioc(m),
            //           242 => self.di(m),
            //           243 => self.xx(m),
            //           244 => self.pushaf(m),
            //           245 => self.xorn(m),
            //           246 => self.rst30(m),
            //           247 => self.ldhlspn(m),
            //           248 => self.xx(m),
            //           249 => self.ldamm(m),
            //           250 => self.ei(m),
            //           251 => self.xx(m),
            //           252 => self.xx(m),
            //           253 => self.cpn(m),
            //           254 => self.rst38(m),
            _ => self.nop(),
        }
    }
}
