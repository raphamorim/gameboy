use crate::cpu::cpu::Cpu;

pub fn rla(cpu: &mut Cpu) {
    let mut ci = 0;
    let mut co = 0;
    if (cpu.registers.f & 0x10) > 0 {
        ci = 1;
    }
    if (cpu.registers.a & 0x80) > 0 {
        co = 0x10;
    }
    cpu.registers.a = (cpu.registers.a << 1) + ci;
    cpu.registers.f = (cpu.registers.f & 0xEF) + co;
}
pub fn rlca(cpu: &mut Cpu) {
    let mut ci = 0;
    let mut co = 0;
    if (cpu.registers.a & 0x80) > 0 {
        ci = 1;
        co = 0x10;
    }

    cpu.registers.a = (cpu.registers.a << 1) + ci;
    cpu.registers.f = (cpu.registers.f & 0xEF) + co;
}
pub fn rra(cpu: &mut Cpu) {
    let mut ci = 0;
    let mut co = 0;
    if (cpu.registers.f & 0x10) > 0 {
        ci = 0x80;
    }
    if (cpu.registers.a & 1) > 0 {
        co = 0x10;
    }

    cpu.registers.a = (cpu.registers.a >> 1) + ci;
    cpu.registers.f = (cpu.registers.f & 0xEF) + co;
}
pub fn rrca(cpu: &mut Cpu) {
    let mut ci = 0;
    let mut co = 0;
    if (cpu.registers.a & 1) > 0 {
        ci = 0x80;
        co = 0x10;
    }

    cpu.registers.a = (cpu.registers.a >> 1) + ci;
    cpu.registers.f = (cpu.registers.f & 0xEF) + co;
}
