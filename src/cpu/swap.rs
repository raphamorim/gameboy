use crate::cpu::cpu::Cpu;

pub fn r_b(cpu: &mut Cpu) {
    let tr = cpu.registers.b;
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.b = cpu.memory.rb(addr);
    let waddr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(waddr, tr);
}
pub fn r_c(cpu: &mut Cpu) {
    let tr = cpu.registers.c;
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.c = cpu.memory.rb(addr);
    let waddr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(waddr, tr);
}
pub fn r_d(cpu: &mut Cpu) {
    let tr = cpu.registers.d;
    cpu.registers.d = cpu
        .memory
        .rb(((cpu.registers.h as u16) << 8) + cpu.registers.l as u16);
    cpu.memory
        .wb(((cpu.registers.h as u16) << 8) + cpu.registers.l as u16, tr);
}
pub fn r_e(cpu: &mut Cpu) {
    let tr = cpu.registers.e;
    cpu.registers.e = cpu
        .memory
        .rb(((cpu.registers.h as u16) << 8) + cpu.registers.l as u16);
    cpu.memory
        .wb(((cpu.registers.h as u16) << 8) + cpu.registers.l as u16, tr);
}
pub fn r_h(cpu: &mut Cpu) {
    let tr = cpu.registers.h;
    cpu.registers.h = cpu
        .memory
        .rb(((cpu.registers.h as u16) << 8) + cpu.registers.l as u16);
    cpu.memory
        .wb(((cpu.registers.h as u16) << 8) + cpu.registers.l as u16, tr);
}
pub fn r_l(cpu: &mut Cpu) {
    let tr = cpu.registers.l;
    cpu.registers.l = cpu
        .memory
        .rb(((cpu.registers.h as u16) << 8) + cpu.registers.l as u16);
    cpu.memory
        .wb(((cpu.registers.h as u16) << 8) + cpu.registers.l as u16, tr);
}
pub fn r_a(cpu: &mut Cpu) {
    let tr = cpu.registers.a;
    cpu.registers.a = cpu
        .memory
        .rb(((cpu.registers.h as u16) << 8) + cpu.registers.l as u16);
    cpu.memory
        .wb(((cpu.registers.h as u16) << 8) + cpu.registers.l as u16, tr);
}
