use crate::cpu::cpu::Cpu;

pub fn bb(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.b;
}
pub fn bc(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.c;
}
pub fn bd(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.d;
}
pub fn be(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.e;
}
pub fn bh(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.h;
}
pub fn bl(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.l;
}
pub fn ba(cpu: &mut Cpu) {
    cpu.registers.b = cpu.registers.a;
}
pub fn cb(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.b;
}
pub fn cd(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.d;
}
pub fn ce(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.e;
}
pub fn ch(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.h;
}
pub fn cl(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.l;
}
pub fn ca(cpu: &mut Cpu) {
    cpu.registers.c = cpu.registers.a;
}
pub fn db(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.b;
}
pub fn dc(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.c;
}
pub fn de(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.e;
}
pub fn dh(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.h;
}
pub fn dl(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.l;
}
pub fn da(cpu: &mut Cpu) {
    cpu.registers.d = cpu.registers.a;
}
pub fn eb(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.b;
}
pub fn ec(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.c;
}
pub fn ed(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.d;
}
pub fn eh(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.h;
}
pub fn el(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.l;
}
pub fn ea(cpu: &mut Cpu) {
    cpu.registers.e = cpu.registers.a;
}
pub fn hb(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.b;
}
pub fn hc(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.c;
}
pub fn hd(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.d;
}
pub fn he(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.e;
}
pub fn hh(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.h;
}
pub fn hl(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.l;
}
pub fn ha(cpu: &mut Cpu) {
    cpu.registers.h = cpu.registers.a;
}
pub fn lb(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.b;
}
pub fn lc(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.c;
}
pub fn ld(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.d;
}
pub fn le(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.e;
}
pub fn lh(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.h;
}
pub fn la(cpu: &mut Cpu) {
    cpu.registers.l = cpu.registers.a;
}
pub fn ab(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.b;
}
pub fn ac(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.c;
}
pub fn ad(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.d;
}
pub fn ae(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.e;
}
pub fn ah(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.h;
}
pub fn al(cpu: &mut Cpu) {
    cpu.registers.a = cpu.registers.l;
}
pub fn b(cpu: &mut Cpu) {
    cpu.registers.b = cpu.get_byte();
}
pub fn c(cpu: &mut Cpu) {
    cpu.registers.c = cpu.get_byte();
}
pub fn d(cpu: &mut Cpu) {
    cpu.registers.d = cpu.get_byte();
}
pub fn e(cpu: &mut Cpu) {
    cpu.registers.e = cpu.get_byte();
}
pub fn h(cpu: &mut Cpu) {
    cpu.registers.h = cpu.get_byte();
}
pub fn l(cpu: &mut Cpu) {
    cpu.registers.l = cpu.get_byte();
}
pub fn a(cpu: &mut Cpu) {
    cpu.registers.a = cpu.get_byte();
}
