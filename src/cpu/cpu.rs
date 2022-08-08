use crate::cpu::bit;
use crate::cpu::data;
use crate::cpu::registers::{Clock, Registers};
use crate::cpu::registers::CpuFlag::{C, H, N, Z};
use crate::cpu::stack;
use crate::mmu::mmu::Mmu;

#[derive(Debug)]
pub struct Cpu {
    pub _r: Registers, // registers
    pub clock: Clock,
    stop_loop: bool,
    _halt: u8,
    cycles: u32,
    instructions: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            _r: Registers::new(),
            _halt: 0,
            stop_loop: false,
            cycles: 0,
            clock: Clock { m: 0, t: 0 },
            instructions: Vec::new(),
        }
    }
    pub fn get_byte(&mut self, m: &mut Mmu) -> u8 {
        let pc = m.r8b(self._r.pc);
        self._r.pc = self._r.pc.wrapping_add(1);
        pc
    }

    pub fn get_word(&mut self, m: &mut Mmu) -> u16 {
        let w = m.r16b(self._r.pc);
        self._r.pc += 2;
        w
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

    // fz: function(i,as) {
    // Z80._r.f=0;
    // if(!(i&255)) {
    //    Z80._r.f|=128;
    // }
    // Z80._r.f|=as?0x40:0;
    // }
    pub fn fz(&mut self, i: u8, cond: u8) {
        self._r.f = 0;
        if !((i & 255) > 0) {
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
        println!(
            "Instruction at {:#01x} | {:#06x} | {} not implemented, stopping.",
            opc, opc, opc
        );
        self.stop_loop = true;
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
        let a = self._r.h;
        let b = 0;
        let r = a & (1 << (b as u32)) == 0;
        self._r.flag(N, false);
        self._r.flag(H, true);
        self._r.flag(Z, r);

        // self._r.b = self._r.h;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn ldrr_bl(&mut self) {
        self._r.b = self._r.l;
        self._r.m = 1;
        self._r.t = 4;
    }
    // to change
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
    fn sbcn(&mut self, m: &mut Mmu) {
        // self._r.a = self._r.a;
        self._r.a -= m.r8b(self._r.pc);
        self._r.pc += 1;
        // self._r.a-=(self._r.f&0x10)?1:0;
        if (self._r.f & 0x10) > 0 {
            self._r.a -= 1;
        }
        self.fz(self._r.a, 1);
        // if self._r.a < 0 {
        //     self._r.f|=0x10;
        // }
        self._r.m = 2;
        self._r.t = 8;
    }

    fn ldr_hlm_b(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.b = m.r8b(addr);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_c(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.c = m.r8b(addr);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_d(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.d = m.r8b(addr);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_e(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.e = m.r8b(addr);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_h(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.h = m.r8b(addr);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_l(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.l = m.r8b(addr);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldr_hlm_a(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.a = m.r8b(addr);
        self._r.m = 2;
        self._r.t = 8;
    }

    fn ld_hlmr_b(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(addr, self._r.b);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_c(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(addr, self._r.c);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_d(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(addr, self._r.d);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_e(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(addr, self._r.e);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_h(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(addr, self._r.h);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_l(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(addr, self._r.l);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmr_a(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(addr, self._r.a);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_b(&mut self, m: &mut Mmu) {
        self._r.b = self.get_byte(m);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_c(&mut self, m: &mut Mmu) {
        self._r.c = self.get_byte(m);
        self._r.m = 2;
        self._r.t = 8;
    }
    // To change
    fn ldrn_d(&mut self, m: &mut Mmu) {
        self._r.d = self.get_byte(m);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_e(&mut self, m: &mut Mmu) {
        self._r.e = self.get_byte(m);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_h(&mut self, m: &mut Mmu) {
        self._r.h = self.get_byte(m);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_l(&mut self, m: &mut Mmu) {
        self._r.l = self.get_byte(m);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldrn_a(&mut self, m: &mut Mmu) {
        self._r.a = self.get_byte(m);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_hlmn(&mut self, m: &mut Mmu) {
        let value = m.r8b(self._r.pc);
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(addr, value);
        self._r.pc += 1;
        self._r.m = 3;
        self._r.t = 12;
    }
    fn ld_bcm_a(&mut self, m: &mut Mmu) {
        let addr = ((self._r.b as u16) << 8) + self._r.c as u16;
        m.w8b(addr, self._r.a);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_dem_a(&mut self, m: &mut Mmu) {
        let addr = ((self._r.d as u16) << 8) + self._r.e as u16;
        m.w8b(addr, self._r.a);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ldmm_a(&mut self, m: &mut Mmu) {
        // let addr = m.r16b(self._r.pc);
        // m.w8b(addr, self._r.a);
        // self._r.pc += 2;
        let a = self.get_word(m);
        m.w8b(a, self._r.a);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn ld_abcm(&mut self, m: &mut Mmu) {
        let addr = ((self._r.b as u16) << 8) + self._r.c as u16;
        self._r.a = m.r8b(addr);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_adem(&mut self, m: &mut Mmu) {
        let addr = ((self._r.d as u16) << 8) + self._r.e as u16;
        self._r.a = m.r8b(addr);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_amm(&mut self, m: &mut Mmu) {
        let a = self.get_word(m);
        self._r.a = m.r8b(a);

        // let addr = m.r16b(self._r.pc);
        // self._r.a = m.r8b(addr);
        // self._r.pc += 2;
        self._r.m = 4;
        self._r.t = 16;
    }
    fn ld_bcnn(&mut self, m: &mut Mmu) {
        let value = self.get_word(m);
        self._r.b = (value >> 8) as u8;
        self._r.c = (value & 0x00FF) as u8;
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
        let v = self.get_word(m);
        self._r.h = (v >> 8) as u8;
        self._r.l = (v & 0x00FF) as u8;
        self._r.m = 3;
        self._r.t = 12;
    }
    fn ld_spnn(&mut self, m: &mut Mmu) {
        self._r.sp = self.get_word(m);
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
        let value = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w16b(addr, value);
        self._r.m = 5;
        self._r.t = 20;
    }
    fn ld_hlia(&mut self, m: &mut Mmu) {
        let mut hl = ((self._r.h as u16) << 8) | (self._r.l as u16);
        hl += 1;
        self._r.h = (hl >> 8) as u8;
        self._r.l = (hl & 0x00FF) as u8;
        m.w8b(hl, self._r.a);
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_ahli(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.a = m.r8b(addr);
        self._r.l = (self._r.l + 1) & 255;
        if self._r.l == 0 {
            self._r.h = (self._r.h + 1) & 255;
        }
        self._r.m = 2;
        self._r.t = 8;
    }

    fn ld_hld_a(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(addr, self._r.a);
        self._r.l = (self._r.l - 1) & 255;
        if self._r.l == 255 {
            self._r.h = (self._r.h - 1) & 255;
        }
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_ahld(&mut self, m: &mut Mmu) {
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.a = m.r8b(addr);
        self._r.l = (self._r.l - 1) & 255;
        if self._r.l == 255 {
            self._r.h = (self._r.h - 1) & 255;
        }
        self._r.m = 2;
        self._r.t = 8;
    }
    fn ld_aion(&mut self, m: &mut Mmu) {
        let addr = 0xFF00 | self.get_byte(m) as u16;
        self._r.a = m.r8b(addr);
        self._r.m = 3;
        self._r.t = 12;
    }
    fn ld_ion_a(&mut self, m: &mut Mmu) {
        // let addr = ((0xFF00 as u32 as i32) + (m.r16b(self._r.pc) as i32)) as u16;
        // m.w8b(addr, self._r.a);
        let a = 0xFF00 | self.get_byte(m) as u16;
        m.w8b(a, self._r.a);
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
        let mut i: u8 = m.r8b(self._r.pc);
        if i > 127 {
            // i=-(!i+1);
            // i = (i - (self._r.sp as u8)) + 1;
            i = 1;
        }
        self._r.pc += 1;
        i += self._r.sp as u8;
        // self._r.h = ((i >> 8) as u8) & 255;
        self._r.h = i;
        self._r.l = i & 255;
        self._r.m = 3;
        self._r.t = 12;
    }
    fn ldmmsp(&mut self, m: &mut Mmu) {
        let addr = self.get_word(m);
        m.w16b(addr, self._r.sp);
    }
    fn swapr_b(&mut self, m: &mut Mmu) {
        let tr = self._r.b;
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.b = m.r8b(addr);
        let waddr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(waddr, tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_c(&mut self, m: &mut Mmu) {
        let tr = self._r.c;
        let addr = ((self._r.h as u16) << 8) + self._r.l as u16;
        self._r.c = m.r8b(addr);
        let waddr = ((self._r.h as u16) << 8) + self._r.l as u16;
        m.w8b(waddr, tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_d(&mut self, m: &mut Mmu) {
        let tr = self._r.d;
        self._r.d = m.r8b(((self._r.h as u16) << 8) + self._r.l as u16);
        m.w8b(((self._r.h as u16) << 8) + self._r.l as u16, tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_e(&mut self, m: &mut Mmu) {
        let tr = self._r.e;
        self._r.e = m.r8b(((self._r.h as u16) << 8) + self._r.l as u16);
        m.w8b(((self._r.h as u16) << 8) + self._r.l as u16, tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_h(&mut self, m: &mut Mmu) {
        let tr = self._r.h;
        self._r.h = m.r8b(((self._r.h as u16) << 8) + self._r.l as u16);
        m.w8b(((self._r.h as u16) << 8) + self._r.l as u16, tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_l(&mut self, m: &mut Mmu) {
        let tr = self._r.l;
        self._r.l = m.r8b(((self._r.h as u16) << 8) + self._r.l as u16);
        m.w8b(((self._r.h as u16) << 8) + self._r.l as u16, tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn swapr_a(&mut self, m: &mut Mmu) {
        let tr = self._r.a;
        self._r.a = m.r8b(((self._r.h as u16) << 8) + self._r.l as u16);
        m.w8b(((self._r.h as u16) << 8) + self._r.l as u16, tr);
        self._r.m = 4;
        self._r.t = 16;
    }
    fn rla(&mut self) {
        // let ci=self._r.f&0x10?1:0;
        let mut ci = 0;
        let mut co = 0;
        if (self._r.f & 0x10) > 0 {
            ci = 1;
        }
        if (self._r.a & 0x80) > 0 {
            co = 0x10;
        }
        self._r.a = (self._r.a << 1) + ci;
        self._r.f = (self._r.f & 0xEF) + co;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn rlca(&mut self) {
        let mut ci = 0;
        let mut co = 0;
        if (self._r.a & 0x80) > 0 {
            ci = 1;
            co = 0x10;
        }

        self._r.a = (self._r.a << 1) + ci;
        self._r.f = (self._r.f & 0xEF) + co;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn rra(&mut self) {
        let mut ci = 0;
        let mut co = 0;
        if (self._r.f & 0x10) > 0 {
            ci = 0x80;
        }
        if (self._r.a & 1) > 0 {
            co = 0x10;
        }

        self._r.a = (self._r.a >> 1) + ci;
        self._r.f = (self._r.f & 0xEF) + co;
        self._r.m = 1;
        self._r.t = 4;
    }
    fn rrca(&mut self) {
        let mut ci = 0;
        let mut co = 0;
        if (self._r.a & 1) > 0 {
            ci = 0x80;
            co = 0x10;
        }

        self._r.a = (self._r.a >> 1) + ci;
        self._r.f = (self._r.f & 0xEF) + co;
        self._r.m = 1;
        self._r.t = 4;
    }
    pub fn mapcb(&mut self, m: &mut Mmu) {
        let i = self.get_byte(m);
        self.cbmap(i, m);
    }

    fn mut_find_or_insert<T: PartialEq>(vec: &mut Vec<T>, val: T) -> &mut T {
        if let Some(i) = vec.iter().position(|each| *each == val) {
            &mut vec[i]
        } else {
            vec.push(val);
            vec.last_mut().unwrap()
        }
    }

    pub fn debug_instructions(&mut self, ins: u8) {
        Cpu::mut_find_or_insert(&mut self.instructions, ins);
    }

    pub fn exec(&mut self, m: &mut Mmu) {
        self.clock.t = 0;
        self.clock.m = 0;
        while self.clock.t <= 70224 {
            // in case self.xx is called
            if self.stop_loop {
                break;
            }

            let counter = self.get_byte(m);
            // println!("{:?} | {:#01x} | {:?}", counter, counter, self._r);
            println!("{:?} {:?}", counter, self._r);
            self.debug_instructions(counter);
            self.program_counter_call(counter, m);
            let time = self._r.t as u32;
            // m.timer.step(time);
            m.gpu.step(time);
            self.clock.inc_m(self._r.m);
            self.clock.inc_t(self._r.t);
        }

        println!("Instructions used: {:?}", self.instructions);
    }

    fn program_counter_call(&mut self, op: u8, m: &mut Mmu) {
        // println!("{:#01x} op", op);
        match op {
            0x00 => self.nop(),      // ok
            0x01 => self.ld_bcnn(m), // ok
            0x02 => self.ld_bcm_a(m),
            0x03 => data::incbc(self),
            0x04 => data::incr_b(self),
            0x05 => data::decr_b(self),
            0x06 => self.ldrn_b(m),
            0x07 => self.rlca(),
            0x08 => self.ldmmsp(m),
            0x09 => data::addhlbc(self),
            0x0A => self.ld_abcm(m),
            11 => data::decbc(self),
            12 => data::incr_c(self),
            13 => data::decr_c(self),
            14 => self.ldrn_c(m),
            15 => self.rrca(),
            0x10 => stack::djnzn(self, m), // Switch speed
            0x11 => self.ld_denn(m),
            18 => self.ld_dem_a(m),
            19 => data::incde(self),
            20 => data::incr_d(self),
            0x15 => data::decr_d(self), // ok
            22 => self.ldrn_d(m),
            23 => self.rla(),
            0x18 => stack::jrn(self, m),
            0x19 => data::addhlde(self),
            26 => self.ld_adem(m),
            27 => data::decde(self),
            28 => data::incr_e(self),
            29 => data::decr_e(self),
            30 => self.ldrn_e(m),
            31 => self.rra(),
            0x20 => stack::jrnzn(self, m), // ok
            33 => self.ld_hlnn(m),
            0x22 => self.ld_hlia(m),
            35 => data::inchl(self),
            36 => data::incr_h(self),
            37 => data::decr_h(self),
            38 => self.ldrn_h(m),
            39 => self.xx(),
            0x28 => stack::jrzn(self, m),
            41 => data::addhlhl(self),
            42 => self.ld_ahli(m),
            43 => data::dechl(self),
            44 => data::incr_l(self),
            45 => data::decr_l(self),
            46 => self.ldrn_l(m),
            0x2f => data::cpl(self),
            48 => stack::jrncn(self, m),
            0x31 => self.ld_spnn(m), // ok
            50 => self.ld_hld_a(m),
            51 => data::incsp(self),
            52 => data::inchlm(self, m),
            53 => data::dechlm(self, m),
            54 => self.ld_hlmn(m),
            0x37 => data::scf(self),
            0x38 => stack::jrcn(self, m),
            57 => data::addhlsp(self),
            58 => self.ld_ahld(m),
            59 => data::decsp(self),
            60 => data::incr_a(self),
            61 => data::decr_a(self),
            0x3e => self.ldrn_a(m), // ok
            0x3F => data::ccf(self),
            64 => self.ldrr_bb(),
            65 => self.ldrr_bc(),
            66 => self.ldrr_bd(),
            67 => self.ldrr_be(),
            68 => self.ldrr_bh(),
            69 => self.ldrr_bl(),
            70 => self.ldr_hlm_b(m),
            71 => self.ldrr_ba(),
            72 => self.ldrr_cb(),
            73 => self.ldrr_cc(),
            74 => self.ldrr_cd(),
            75 => self.ldrr_ce(),
            76 => self.ldrr_ch(),
            77 => self.ldrr_cl(),
            78 => self.ldr_hlm_c(m),
            79 => self.ldrr_ca(),
            80 => self.ldrr_db(),
            81 => self.ldrr_dc(),
            82 => self.ldrr_dd(),
            83 => self.ldrr_de(),
            84 => self.ldrr_dh(),
            85 => self.ldrr_dl(),
            86 => self.ldr_hlm_d(m),
            87 => self.ldrr_da(),
            88 => self.ldrr_eb(),
            89 => self.ldrr_ec(),
            90 => self.ldrr_ed(),
            91 => self.ldrr_ee(),
            92 => self.ldrr_eh(),
            93 => self.ldrr_el(),
            94 => self.ldr_hlm_e(m),
            95 => self.ldrr_ea(),
            96 => self.ldrr_hb(),
            97 => self.ldrr_hc(),
            98 => self.ldrr_hd(),
            99 => self.ldrr_he(),
            100 => self.ldrr_hh(),
            101 => self.ldrr_hl(),
            102 => self.ldr_hlm_h(m),
            103 => self.ldrr_ha(),
            104 => self.ldrr_lb(),
            105 => self.ldrr_lc(),
            106 => self.ldrr_ld(),
            107 => self.ldrr_le(),
            108 => self.ldrr_lh(),
            109 => self.ldrr_ll(),
            110 => self.ldr_hlm_l(m),
            111 => self.ldrr_la(),
            112 => self.ld_hlmr_b(m),
            113 => self.ld_hlmr_c(m),
            114 => self.ld_hlmr_d(m),
            115 => self.ld_hlmr_e(m),
            116 => self.ld_hlmr_h(m),
            117 => self.ld_hlmr_l(m),
            118 => self.halt(),
            119 => self.ld_hlmr_a(m),
            120 => self.ldrr_ab(),
            121 => self.ldrr_ac(),
            122 => self.ldrr_ad(),
            123 => self.ldrr_ae(),
            124 => self.ldrr_ah(),
            125 => self.ldrr_al(),
            126 => self.ldr_hlm_a(m),
            127 => self.ldrr_aa(),
            128 => data::addr_b(self),
            129 => data::addr_c(self),
            130 => data::addr_d(self),
            131 => data::addr_e(self),
            132 => data::addr_h(self),
            133 => data::addr_l(self),
            134 => data::addhl(self, m),
            135 => data::addr_a(self),
            136 => data::adcr_b(self),
            137 => data::adcr_c(self),
            138 => data::adcr_d(self),
            139 => data::adcr_e(self),
            140 => data::adcr_h(self),
            141 => data::adcr_l(self),
            142 => data::adchl(self, m),
            143 => data::adcr_a(self),
            144 => data::subr_b(self),
            0x91 => data::subr_c(self),
            146 => data::subr_d(self),
            147 => data::subr_e(self),
            148 => data::subr_h(self),
            149 => data::subr_l(self),
            150 => data::subhl(self, m),
            0x97 => data::subr_a(self), // ok
            152 => data::sbcr_b(self),
            153 => data::sbcr_c(self),
            154 => data::sbcr_d(self),
            155 => data::sbcr_e(self),
            156 => data::sbcr_h(self),
            157 => data::sbcr_l(self),
            158 => data::sbchl(self, m),
            159 => data::sbcr_a(self),
            160 => data::andr_b(self),
            161 => data::andr_c(self),
            162 => data::andr_d(self),
            163 => data::andr_e(self),
            164 => data::andr_h(self),
            165 => data::andr_l(self),
            166 => data::andhl(self, m),
            167 => data::andr_a(self),
            168 => data::xorr_b(self),
            169 => data::xorr_c(self),
            170 => data::xorr_d(self),
            171 => data::xorr_e(self),
            172 => data::xorr_h(self),
            173 => data::xorr_l(self),
            174 => data::xorhl(self, m),
            175 => data::xorr_a(self),
            176 => data::orr_b(self),
            177 => data::orr_c(self),
            178 => data::orr_d(self),
            179 => data::orr_e(self),
            180 => data::orr_h(self),
            181 => data::orr_l(self),
            182 => data::orhl(self, m),
            183 => data::orr_a(self),
            184 => data::cpr_b(self),
            185 => data::cpr_c(self),
            186 => data::cpr_d(self),
            187 => data::cpr_e(self),
            188 => data::cpr_h(self),
            189 => data::cpr_l(self),
            190 => data::cphl(self, m),
            191 => data::cpr_a(self),
            192 => stack::retnz(self, m),
            193 => stack::popbc(self, m),
            0xc2 => stack::jpnznn(self, m), // ok
            0xc3 => stack::jpnn(self, m),   // ok
            196 => stack::callnznn(self, m),
            197 => stack::pushbc(self, m),
            198 => data::addn(self, m),
            199 => stack::rst00(self, m),
            200 => stack::retz(self, m),
            201 => stack::ret(self, m), // ok
            202 => stack::jpznn(self, m),
            0xcb => self.mapcb(m),
            204 => stack::callznn(self, m),
            0xcd => stack::callnn(self, m),
            206 => data::adcn(self, m),
            207 => stack::rst08(self, m),
            208 => stack::retnc(self, m),
            209 => stack::popde(self, m),
            210 => stack::jpncnn(self, m),
            211 => self.xx(),
            212 => stack::callncnn(self, m),
            213 => stack::pushde(self, m),
            214 => data::subn(self, m),
            215 => stack::rst10(self, m),
            216 => stack::retc(self, m),
            217 => stack::reti(self, m),
            218 => stack::jpcnn(self, m),
            219 => self.xx(),
            220 => stack::callcnn(self, m),
            221 => self.xx(),
            0xDE => self.sbcn(m),
            223 => stack::rst18(self, m),
            0xe0 => self.ld_ion_a(m), // ok
            225 => stack::pophl(self, m),
            226 => self.ld_ioca(m),
            227 => self.xx(),
            228 => self.xx(),
            229 => stack::pushhl(self, m),
            0xe6 => data::andn(self, m),
            231 => stack::rst20(self, m),
            232 => data::addspn(self, m),
            233 => stack::jphl(self),
            0xea => self.ldmm_a(m),
            235 => self.xx(),
            236 => self.xx(),
            237 => self.xx(),
            238 => data::orn(self, m),
            239 => stack::rst28(self, m),
            0xf0 => self.ld_aion(m), // ok
            241 => stack::popaf(self, m),
            242 => self.ld_aioc(m),
            243 => self.di(),
            244 => self.xx(),
            245 => stack::pushaf(self, m),
            246 => data::xorn(self, m),
            247 => stack::rst30(self, m),
            248 => self.ld_hlspn(m),
            249 => self.xx(),
            250 => self.ld_amm(m),
            251 => self.ei(),
            252 => self.xx(),
            253 => self.xx(),
            254 => data::cpn(self, m),
            0xff => stack::rst38(self, m),
            // _ => self.xx(),
        }
    }

    fn cbmap(&mut self, op: u8, m: &mut Mmu) {
        println!("cbmap {:?}", op);
        match op {
            // CB00
            // 0 => self.RLCr_b,
            // 1 => self.RLCr_c,
            // 2 => self.RLCr_d,
            // 3 => self.RLCr_e,
            // 4 => self.RLCr_h,
            // 5 => self.RLCr_l,
            // 6 => self.RLCHL,
            // 7 => self.RLCr_a,
            // 8 => self.RRCr_b,
            // 9 => self.RRCr_c,
            // 10 => self.RRCr_d,
            // 11 => self.RRCr_e,
            // 12 => self.RRCr_h,
            // 13 => self.RRCr_l,
            // 14 => self.RRCHL,
            // 15 => self.RRCr_a,
            // 16 => self.RLr_b,
            // 17 => self.RLr_c,
            // 18 => self.RLr_d,
            // 19 => self.RLr_e,
            // 20 => self.RLr_h,
            // 21 => self.RLr_l,
            // 22 => self.RLHL,
            // 23 => self.RLr_a,
            // 24 => self.RRr_b,
            // 25 => self.RRr_c,
            // 26 => self.RRr_d,
            // 27 => self.RRr_e,
            // 28 => self.RRr_h,
            // 29 => self.RRr_l,
            // 30 => self.RRHL,
            // 31 => self.RRr_a,
            // 32 => self.SLAr_b,
            // 33 => self.SLAr_c,
            // 34 => self.SLAr_d,
            // 35 => self.SLAr_e,
            // 36 => self.SLAr_h,
            // 37 => self.SLAr_l,
            // 38 => self.xx(),
            // 39 => self.SLAr_a,
            // 40 => self.SRAr_b,
            // 41 => self.SRAr_c,
            // 42 => self.SRAr_d,
            // 43 => self.SRAr_e,
            // 44 => self.SRAr_h,
            // 45 => self.SRAr_l,
            46 => self.xx(),
            // 47 => self.SRAr_a,
            48 => self.swapr_b(m),
            49 => self.swapr_c(m),
            50 => self.swapr_d(m),
            51 => self.swapr_e(m),
            52 => self.swapr_h(m),
            53 => self.swapr_l(m),
            54 => self.xx(),
            55 => self.swapr_a(m),
            // 56 => self.SRLr_b,
            // 57 => self.SRLr_c,
            // 58 => self.SRLr_d,
            // 59 => self.SRLr_e,
            // 60 => self.SRLr_h,
            // 61 => self.SRLr_l,
            // 62 => self.xx(),
            // 63 => self.SRLr_a,
            64 => bit::bit0b(self),
            65 => bit::bit0c(self),
            66 => bit::bit0d(self),
            67 => bit::bit0e(self),
            68 => bit::bit0h(self),
            69 => bit::bit0l(self),
            70 => bit::bit0m(self, m),
            71 => bit::bit0a(self),
            72 => bit::bit1b(self),
            73 => bit::bit1c(self),
            74 => bit::bit1d(self),
            75 => bit::bit1e(self),
            76 => bit::bit1h(self),
            77 => bit::bit1l(self),
            78 => bit::bit1m(self, m),
            79 => bit::bit1a(self),
            80 => bit::bit2b(self),
            81 => bit::bit2c(self),
            82 => bit::bit2d(self),
            83 => bit::bit2e(self),
            84 => bit::bit2h(self),
            85 => bit::bit2l(self),
            86 => bit::bit2m(self, m),
            87 => bit::bit2a(self),
            88 => bit::bit3b(self),
            89 => bit::bit3c(self),
            90 => bit::bit3d(self),
            91 => bit::bit3e(self),
            92 => bit::bit3h(self),
            93 => bit::bit3l(self),
            94 => bit::bit3m(self, m),
            95 => bit::bit3a(self),

            // CB60
            96 => bit::bit4b(self),
            97 => bit::bit4c(self),
            98 => bit::bit4d(self),
            99 => bit::bit4e(self),
            100 => bit::bit4h(self),
            101 => bit::bit4l(self),
            102 => bit::bit4m(self, m),
            103 => bit::bit4a(self),
            104 => bit::bit5b(self),
            105 => bit::bit5c(self),
            106 => bit::bit5d(self),
            107 => bit::bit5e(self),
            108 => bit::bit5h(self),
            109 => bit::bit5l(self),
            110 => bit::bit5m(self, m),
            111 => bit::bit5a(self),
            112 => bit::bit6b(self),
            113 => bit::bit6c(self),
            114 => bit::bit6d(self),
            115 => bit::bit6e(self),
            116 => bit::bit6h(self),
            117 => bit::bit6l(self),
            118 => bit::bit6m(self, m),
            119 => bit::bit6a(self),
            120 => bit::bit7b(self),
            121 => bit::bit7c(self),
            122 => bit::bit7d(self),
            123 => bit::bit7e(self),
            124 => bit::bit7h(self),
            125 => bit::bit7l(self),
            126 => bit::bit7m(self, m),
            127 => bit::bit7a(self),
            _ => {
                println!("cbmap");
                self.xx()
            }
        }
    }
}
