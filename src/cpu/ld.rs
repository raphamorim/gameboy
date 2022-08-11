pub fn r_hlm_b(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.b = cpu.memory.rb(addr);
}
pub fn r_hlm_c(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.c = cpu.memory.rb(addr);
}
pub fn r_hlm_d(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.d = cpu.memory.rb(addr);
}
pub fn r_hlm_e(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.e = cpu.memory.rb(addr);
}
pub fn r_hlm_h(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.h = cpu.memory.rb(addr);
}
pub fn r_hlm_l(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.l = cpu.memory.rb(addr);
}
pub fn r_hlm_a(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.a = cpu.memory.rb(addr);
}
pub fn hlmr_b(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(addr, cpu.registers.b);
}
pub fn hlmr_c(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(addr, cpu.registers.c);
}
pub fn hlmr_d(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(addr, cpu.registers.d);
}
pub fn hlmr_e(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(addr, cpu.registers.e);
}
pub fn hlmr_h(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(addr, cpu.registers.h);
}
pub fn hlmr_l(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(addr, cpu.registers.l);
}
pub fn hlmr_a(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(addr, cpu.registers.a);
}
pub fn hlmn(cpu: &mut Cpu) {
    let value = cpu.memory.rb(cpu.registers.pc);
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(addr, value);
    cpu.registers.pc += 1;
}
pub fn bcm_a(cpu: &mut Cpu) {
    let addr = ((cpu.registers.b as u16) << 8) + cpu.registers.c as u16;
    cpu.memory.wb(addr, cpu.registers.a);
}
pub fn dem_a(cpu: &mut Cpu) {
    let addr = ((cpu.registers.d as u16) << 8) + cpu.registers.e as u16;
    cpu.memory.wb(addr, cpu.registers.a);
}
pub fn mm_a(cpu: &mut Cpu) {
    let addr = cpu.get_word();
    cpu.memory.wb(addr, cpu.registers.a);
}
pub fn abcm(cpu: &mut Cpu) {
    let addr = ((cpu.registers.b as u16) << 8) + cpu.registers.c as u16;
    cpu.registers.a = cpu.memory.rb(addr);
}
pub fn adem(cpu: &mut Cpu) {
    let addr = ((cpu.registers.d as u16) << 8) + cpu.registers.e as u16;
    cpu.registers.a = cpu.memory.rb(addr);
}
pub fn amm(cpu: &mut Cpu) {
    let addr = cpu.get_word();
    cpu.registers.a = cpu.memory.rb(addr);
}
pub fn bcnn(cpu: &mut Cpu) {
    let value = cpu.get_word();
    cpu.registers.b = (value >> 8) as u8;
    cpu.registers.c = (value & 0x00FF) as u8;
}
pub fn denn(cpu: &mut Cpu) {
    cpu.registers.e = cpu.memory.rb(cpu.registers.pc);
    cpu.registers.d = cpu.memory.rb(cpu.registers.pc + 1);
    cpu.registers.pc += 2;
}
pub fn hlnn(cpu: &mut Cpu) {
    let v = cpu.get_word();
    cpu.registers.h = (v >> 8) as u8;
    cpu.registers.l = (v & 0x00FF) as u8;
}
pub fn spnn(cpu: &mut Cpu) {
    cpu.registers.sp = cpu.get_word();
}
pub fn hlia(cpu: &mut Cpu) {
    let mut hl = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
    cpu.memory.wb(hl, cpu.registers.a);
    hl += 1;
    cpu.registers.h = (hl >> 8) as u8;
    cpu.registers.l = (hl & 0x00FF) as u8;
}
pub fn ahli(cpu: &mut Cpu) {
    let mut addr = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
    cpu.registers.a = cpu.memory.rb(addr);
    addr += 1;
    cpu.registers.h = (addr >> 8) as u8;
    cpu.registers.l = (addr & 0x00FF) as u8;
}
pub fn hld_a(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(addr, cpu.registers.a);
    cpu.registers.l = (cpu.registers.l - 1) & 255;
    if cpu.registers.l == 255 {
        cpu.registers.h = (cpu.registers.h - 1) & 255;
    }
}
pub fn ahld(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.a = cpu.memory.rb(addr);
    cpu.registers.l = (cpu.registers.l - 1) & 255;
    if cpu.registers.l == 255 {
        cpu.registers.h = (cpu.registers.h - 1) & 255;
    }
}
pub fn aion(cpu: &mut Cpu) {
    let addr = 0xFF00 | cpu.get_byte() as u16;
    cpu.registers.a = cpu.memory.rb(addr);
}
pub fn ion_a(cpu: &mut Cpu) {
    let a = 0xFF00 | cpu.get_byte() as u16;
    cpu.memory.wb(a, cpu.registers.a);
}
pub fn aioc(cpu: &mut Cpu) {
    let addr: u16 = (0xFF00 + cpu.registers.c as u16).into();
    cpu.registers.a = cpu.memory.rb(addr);
}
pub fn ioca(cpu: &mut Cpu) {
    let addr: u16 = (0xFF00 + cpu.registers.c as u16).into();
    cpu.memory.wb(addr, cpu.registers.a);
}
pub fn hlspn(cpu: &mut Cpu) {
    let mut i: u8 = cpu.memory.rb(cpu.registers.pc);
    if i > 127 {
        // i=-(!i+1);
        // i = (i - (cpu.registers.sp as u8)) + 1;
        i = 1;
    }
    cpu.registers.pc += 1;
    i += cpu.registers.sp as u8;
    // cpu.registers.h = ((i >> 8) as u8) & 255;
    cpu.registers.h = i;
    cpu.registers.l = i & 255;
}
pub fn mmsp(cpu: &mut Cpu) {
    let addr = cpu.get_word();
    cpu.memory.ww(addr, cpu.registers.sp);
}

use crate::cpu::cpu::Cpu;

pub fn rr_bb(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.b;
}
pub fn rr_bc(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.c;
}
pub fn rr_bd(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.d;
}
pub fn rr_be(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.e;
}
pub fn rr_bh(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.h;
}
pub fn rr_bl(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.l;
}
pub fn rr_ba(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.a;
}
pub fn rr_cb(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.b;
}
pub fn rr_cd(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.d;
}
pub fn rr_ce(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.e;
}
pub fn rr_ch(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.h;
}
pub fn rr_cl(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.l;
}
pub fn rr_ca(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.a;
}
pub fn rr_db(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.b;
}
pub fn rr_dc(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.c;
}
pub fn rr_de(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.e;
}
pub fn rr_dh(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.h;
}
pub fn rr_dl(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.l;
}
pub fn rr_da(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.a;
}
pub fn rr_eb(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.b;
}
pub fn rr_ec(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.c;
}
pub fn rr_ed(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.d;
}
pub fn rr_eh(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.h;
}
pub fn rr_el(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.l;
}
pub fn rr_ea(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.a;
}
pub fn rr_hb(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.b;
}
pub fn rr_hc(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.c;
}
pub fn rr_hd(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.d;
}
pub fn rr_he(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.e;
}
pub fn rr_hh(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.h;
}
pub fn rr_hl(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.l;
}
pub fn rr_ha(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.a;
}
pub fn rr_lb(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.b;
}
pub fn rr_lc(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.c;
}
pub fn rr_ld(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.d;
}
pub fn rr_le(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.e;
}
pub fn rr_lh(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.h;
}
pub fn rr_la(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.a;
}
pub fn rr_ab(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.b;
}
pub fn rr_ac(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.c;
}
pub fn rr_ad(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.d;
}
pub fn rr_ae(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.e;
}
pub fn rr_ah(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.h;
}
pub fn rr_al(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.l;
}
pub fn rr_b(cpu: &mut Cpu) {
    cpu.registers.b = cpu.get_byte();
}
pub fn rr_c(cpu: &mut Cpu) {
    cpu.registers.c = cpu.get_byte();
}
pub fn rr_d(cpu: &mut Cpu) {
    cpu.registers.d = cpu.get_byte();
}
pub fn rr_e(cpu: &mut Cpu) {
    cpu.registers.e = cpu.get_byte();
}
pub fn rr_h(cpu: &mut Cpu) {
    cpu.registers.h = cpu.get_byte();
}
pub fn rr_l(cpu: &mut Cpu) {
    cpu.registers.l = cpu.get_byte();
}
pub fn rr_a(cpu: &mut Cpu) {
    cpu.registers.a = cpu.get_byte();
}
