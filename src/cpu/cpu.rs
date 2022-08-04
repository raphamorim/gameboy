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

    // fn MAPcb(c: &mut Cpu, m: &mut Mmu) {
    //     let i=m.r8b(Z80._r.pc);
    //     Z80._r.pc += 1;
    //     Z80._r.pc &= 65535;
    //     if Z80._cbmap[i] {
    //         Z80._cbmap[i]();
    //     } else {
    //         alert(i);
    //     }
    // }

    fn ldrr_bb(c: &mut Cpu) {
        c._r.b = c._r.b;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_bc(c: &mut Cpu) {
        c._r.b = c._r.c;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_bd(c: &mut Cpu) {
        c._r.b = c._r.d;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_be(c: &mut Cpu) {
        c._r.b = c._r.e;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_bh(c: &mut Cpu) {
        c._r.b = c._r.h;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_bl(c: &mut Cpu) {
        c._r.b = c._r.l;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ba(c: &mut Cpu) {
        c._r.b = c._r.a;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_cb(c: &mut Cpu) {
        c._r.c = c._r.b;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_cc(c: &mut Cpu) {
        c._r.c = c._r.c;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_cd(c: &mut Cpu) {
        c._r.c = c._r.d;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ce(c: &mut Cpu) {
        c._r.c = c._r.e;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ch(c: &mut Cpu) {
        c._r.c = c._r.h;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_cl(c: &mut Cpu) {
        c._r.c = c._r.l;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ca(c: &mut Cpu) {
        c._r.c = c._r.a;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_db(c: &mut Cpu) {
        c._r.d = c._r.b;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_dc(c: &mut Cpu) {
        c._r.d = c._r.c;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_dd(c: &mut Cpu) {
        c._r.d = c._r.d;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_de(c: &mut Cpu) {
        c._r.d = c._r.e;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_dh(c: &mut Cpu) {
        c._r.d = c._r.h;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_dl(c: &mut Cpu) {
        c._r.d = c._r.l;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_da(c: &mut Cpu) {
        c._r.d = c._r.a;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_eb(c: &mut Cpu) {
        c._r.e = c._r.b;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ec(c: &mut Cpu) {
        c._r.e = c._r.c;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ed(c: &mut Cpu) {
        c._r.e = c._r.d;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ee(c: &mut Cpu) {
        c._r.e = c._r.e;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_eh(c: &mut Cpu) {
        c._r.e = c._r.h;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_el(c: &mut Cpu) {
        c._r.e = c._r.l;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ea(c: &mut Cpu) {
        c._r.e = c._r.a;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_hb(c: &mut Cpu) {
        c._r.h = c._r.b;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_hc(c: &mut Cpu) {
        c._r.h = c._r.c;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_hd(c: &mut Cpu) {
        c._r.h = c._r.d;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_he(c: &mut Cpu) {
        c._r.h = c._r.e;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_hh(c: &mut Cpu) {
        c._r.h = c._r.h;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_hl(c: &mut Cpu) {
        c._r.h = c._r.l;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ha(c: &mut Cpu) {
        c._r.h = c._r.a;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_lb(c: &mut Cpu) {
        c._r.l = c._r.b;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_lc(c: &mut Cpu) {
        c._r.l = c._r.c;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ld(c: &mut Cpu) {
        c._r.l = c._r.d;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_le(c: &mut Cpu) {
        c._r.l = c._r.e;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_lh(c: &mut Cpu) {
        c._r.l = c._r.h;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ll(c: &mut Cpu) {
        c._r.l = c._r.l;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_la(c: &mut Cpu) {
        c._r.l = c._r.a;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ab(c: &mut Cpu) {
        c._r.a = c._r.b;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ac(c: &mut Cpu) {
        c._r.a = c._r.c;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ad(c: &mut Cpu) {
        c._r.a = c._r.d;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ae(c: &mut Cpu) {
        c._r.a = c._r.e;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_ah(c: &mut Cpu) {
        c._r.a = c._r.h;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_al(c: &mut Cpu) {
        c._r.a = c._r.l;
        c._r.m = 1;
        c._r.t = 4;
    }
    fn ldrr_aa(c: &mut Cpu) {
        c._r.a = c._r.a;
        c._r.m = 1;
        c._r.t = 4;
    }

    fn ldr_hlm_b(c: &mut Cpu, m: &mut Mmu) {
        c._r.b = m.rr8b((c._r.h << 8) + c._r.l);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldr_hlm_c(c: &mut Cpu, m: &mut Mmu) {
        c._r.c = m.rr8b((c._r.h << 8) + c._r.l);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldr_hlm_d(c: &mut Cpu, m: &mut Mmu) {
        c._r.d = m.rr8b((c._r.h << 8) + c._r.l);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldr_hlm_e(c: &mut Cpu, m: &mut Mmu) {
        c._r.e = m.rr8b((c._r.h << 8) + c._r.l);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldr_hlm_h(c: &mut Cpu, m: &mut Mmu) {
        c._r.h = m.rr8b((c._r.h << 8) + c._r.l);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldr_hlm_l(c: &mut Cpu, m: &mut Mmu) {
        c._r.l = m.rr8b((c._r.h << 8) + c._r.l);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldr_hlm_a(c: &mut Cpu, m: &mut Mmu) {
        c._r.a = m.rr8b((c._r.h << 8) + c._r.l);
        c._r.m = 2;
        c._r.t = 8;
    }

    fn ld_hlmr_b(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.h << 8) + c._r.l, c._r.b);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_hlmr_c(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.h << 8) + c._r.l, c._r.c);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_hlmr_d(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.h << 8) + c._r.l, c._r.d);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_hlmr_e(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.h << 8) + c._r.l, c._r.e);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_hlmr_h(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.h << 8) + c._r.l, c._r.h);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_hlmr_l(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.h << 8) + c._r.l, c._r.l);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_hlmr_a(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.h << 8) + c._r.l, c._r.a);
        c._r.m = 2;
        c._r.t = 8;
    }

    fn ldrn_b(c: &mut Cpu, m: &mut Mmu) {
        c._r.b = m.r8b(c._r.pc);
        c._r.pc += 1;
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldrn_c(c: &mut Cpu, m: &mut Mmu) {
        c._r.c = m.r8b(c._r.pc);
        c._r.pc += 1;
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldrn_d(c: &mut Cpu, m: &mut Mmu) {
        c._r.d = m.r8b(c._r.pc);
        c._r.pc += 1;
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldrn_e(c: &mut Cpu, m: &mut Mmu) {
        c._r.e = m.r8b(c._r.pc);
        c._r.pc += 1;
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldrn_h(c: &mut Cpu, m: &mut Mmu) {
        c._r.h = m.r8b(c._r.pc);
        c._r.pc += 1;
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldrn_l(c: &mut Cpu, m: &mut Mmu) {
        c._r.l = m.r8b(c._r.pc);
        c._r.pc += 1;
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ldrn_a(c: &mut Cpu, m: &mut Mmu) {
        c._r.a = m.r8b(c._r.pc);
        c._r.pc += 1;
        c._r.m = 2;
        c._r.t = 8;
    }

    fn ld_hlmn(c: &mut Cpu, m: &mut Mmu) {
        let addr = m.r8b(c._r.pc);
        m.ww8b((c._r.h << 8) + c._r.l, addr);
        c._r.pc += 1;
        c._r.m = 3;
        c._r.t = 12;
    }

    fn ld_bcm_a(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.b << 8) + c._r.c, c._r.a);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_dem_a(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.d << 8) + c._r.e, c._r.a);
        c._r.m = 2;
        c._r.t = 8;
    }

    fn ldmm_a(c: &mut Cpu, m: &mut Mmu) {
        let addr = m.r16b(c._r.pc);
        m.w8b(addr, c._r.a);
        c._r.pc += 2;
        c._r.m = 4;
        c._r.t = 16;
    }

    fn ld_abcm(c: &mut Cpu, m: &mut Mmu) {
        c._r.a = m.rr8b((c._r.b << 8) + c._r.c);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_adem(c: &mut Cpu, m: &mut Mmu) {
        c._r.a = m.rr8b((c._r.d << 8) + c._r.e);
        c._r.m = 2;
        c._r.t = 8;
    }

    fn ld_amm(c: &mut Cpu, m: &mut Mmu) {
        let addr = m.r16b(c._r.pc);
        c._r.a = m.r8b(addr);
        c._r.pc += 2;
        c._r.m = 4;
        c._r.t = 16;
    }

    fn ld_bcnn(&mut self, m: &mut Mmu) {
        let c = self;
        c._r.c = m.r8b(c._r.pc);
        c._r.b = m.r8b(c._r.pc + 1);
        c._r.pc += 2;
        c._r.m = 3;
        c._r.t = 12;
    }
    fn ld_denn(c: &mut Cpu, m: &mut Mmu) {
        c._r.e = m.r8b(c._r.pc);
        c._r.d = m.r8b(c._r.pc + 1);
        c._r.pc += 2;
        c._r.m = 3;
        c._r.t = 12;
    }
    fn ld_hlnn(c: &mut Cpu, m: &mut Mmu) {
        c._r.l = m.r8b(c._r.pc);
        c._r.h = m.r8b(c._r.pc + 1);
        c._r.pc += 2;
        c._r.m = 3;
        c._r.t = 12;
    }
    fn ld_spnn(c: &mut Cpu, m: &mut Mmu) {
        c._r.sp = m.r16b(c._r.pc);
        c._r.pc += 2;
        c._r.m = 3;
        c._r.t = 12;
    }

    fn ld_hlmm(c: &mut Cpu, m: &mut Mmu) {
        let i = m.r16b(c._r.pc);
        c._r.pc += 2;
        c._r.l = m.r8b(i);
        c._r.h = m.r8b(i + 1);
        c._r.m = 5;
        c._r.t = 20;
    }
    fn ldmm_hl(c: &mut Cpu, m: &mut Mmu) {
        let addr = m.r16b(c._r.pc);
        c._r.pc += 2;
        let value = (c._r.h << 8) + c._r.l;
        m.ww16b(addr, value);
        c._r.m = 5;
        c._r.t = 20;
    }

    fn ld_hlia(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.h << 8) + c._r.l, c._r.a);
        c._r.l = (c._r.l + 1) & 255;
        if c._r.l == 0 {
            c._r.h = (c._r.h + 1) & 255;
        }
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_ahli(c: &mut Cpu, m: &mut Mmu) {
        c._r.a = m.rr8b((c._r.h << 8) + c._r.l);
        c._r.l = (c._r.l + 1) & 255;
        if c._r.l == 0 {
            c._r.h = (c._r.h + 1) & 255;
        }
        c._r.m = 2;
        c._r.t = 8;
    }

    fn ld_hld_a(c: &mut Cpu, m: &mut Mmu) {
        m.ww8b((c._r.h << 8) + c._r.l, c._r.a);
        c._r.l = (c._r.l - 1) & 255;
        if c._r.l == 255 {
            c._r.h = (c._r.h - 1) & 255;
        }
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_ahld(c: &mut Cpu, m: &mut Mmu) {
        c._r.a = m.rr8b((c._r.h << 8) + c._r.l);
        c._r.l = (c._r.l - 1) & 255;
        if c._r.l == 255 {
            c._r.h = (c._r.h - 1) & 255;
        }
        c._r.m = 2;
        c._r.t = 8;
    }

    fn ld_aion(c: &mut Cpu, m: &mut Mmu) {
        let addr = 0xFF00 + m.r16b(c._r.pc);
        c._r.a = m.r8b(addr);
        c._r.pc += 1;
        c._r.m = 3;
        c._r.t = 12;
    }
    fn ld_ion_a(c: &mut Cpu, m: &mut Mmu) {
        let addr = 0xFF00 + m.r16b(c._r.pc);
        m.w8b(addr, c._r.a);
        c._r.pc += 1;
        c._r.m = 3;
        c._r.t = 12;
    }
    fn ld_aioc(c: &mut Cpu, m: &mut Mmu) {
        let addr: u16 = (0xFF00 + c._r.c as u16).into();
        c._r.a = m.r8b(addr);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_ioca(c: &mut Cpu, m: &mut Mmu) {
        let addr: u16 = (0xFF00 + c._r.c as u16).into();
        m.w8b(addr, c._r.a);
        c._r.m = 2;
        c._r.t = 8;
    }
    fn ld_hlspn(c: &mut Cpu, m: &mut Mmu) {
        let i = m.r8b(c._r.pc);
        if i > 127 {
            // i=-((!i+1)&255);
        }
        c._r.pc += 1;
        // i += c._r.sp.into();
        c._r.h = (i >> 8) & 255;
        c._r.l = i & 255;
        c._r.m = 3;
        c._r.t = 12;
    }

    fn swapr_b(c: &mut Cpu, m: &mut Mmu) {
        let tr = c._r.b;
        let addr = ((c._r.h << 8) + c._r.l) as u16;
        c._r.b = m.r8b(addr);
        let waddr = ((c._r.h << 8) + c._r.l) as u16;
        m.w8b(waddr, tr);
        c._r.m = 4;
        c._r.t = 16;
    }
    fn swapr_c(c: &mut Cpu, m: &mut Mmu) {
        let tr = c._r.c;
        let addr = ((c._r.h << 8) + c._r.l) as u16;
        c._r.c = m.r8b(addr);
        let waddr = ((c._r.h << 8) + c._r.l) as u16;
        m.w8b(waddr, tr);
        c._r.m = 4;
        c._r.t = 16;
    }
    fn swapr_d(c: &mut Cpu, m: &mut Mmu) {
        let tr = c._r.d;
        c._r.d = m.r8b(((c._r.h << 8) + c._r.l).into());
        m.w8b(((c._r.h << 8) + c._r.l).into(), tr);
        c._r.m = 4;
        c._r.t = 16;
    }
    fn swapr_e(c: &mut Cpu, m: &mut Mmu) {
        let tr = c._r.e;
        c._r.e = m.r8b(((c._r.h << 8) + c._r.l).into());
        m.w8b(((c._r.h << 8) + c._r.l).into(), tr);
        c._r.m = 4;
        c._r.t = 16;
    }
    fn swapr_h(c: &mut Cpu, m: &mut Mmu) {
        let tr = c._r.h;
        c._r.h = m.r8b(((c._r.h << 8) + c._r.l).into());
        m.w8b(((c._r.h << 8) + c._r.l).into(), tr);
        c._r.m = 4;
        c._r.t = 16;
    }
    fn swapr_l(c: &mut Cpu, m: &mut Mmu) {
        let tr = c._r.l;
        c._r.l = m.r8b(((c._r.h << 8) + c._r.l).into());
        m.w8b(((c._r.h << 8) + c._r.l).into(), tr);
        c._r.m = 4;
        c._r.t = 16;
    }
    fn swapr_a(c: &mut Cpu, m: &mut Mmu) {
        let tr = c._r.a;
        c._r.a = m.r8b(((c._r.h << 8) + c._r.l).into());
        m.w8b(((c._r.h << 8) + c._r.l).into(), tr);
        c._r.m = 4;
        c._r.t = 16;
    }

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
            self.clock.m += self._r.m; // Add time to CPU clock
            self.clock.t += self._r.t;
        }
    }

    fn program_counter_call(&mut self, op: u8, m: &mut Mmu) {
        match op {
            0 => self.nop(),
            1 => self.ld_bcnn(m),
            2_u8..=u8::MAX => self.nop(),
        }
    }
}
