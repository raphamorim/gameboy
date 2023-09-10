use crate::cpu::cpu::Cpu;
use crate::cpu::registers::CpuFlag::{C, H, N, Z};

fn alu_sub(cpu: &mut Cpu, b: u8, usec: bool) {
    let c = if usec && cpu.registers.getflag(C) {
        1
    } else {
        0
    };
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(c);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F) + c);
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16) + (c as u16));
    cpu.registers.a = r;
}

fn alu_and(cpu: &mut Cpu, b: u8) {
    let r = cpu.registers.a & b;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, true);
    cpu.registers.flag(C, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}

fn alu_add(cpu: &mut Cpu, b: u8, usec: bool) {
    let c = if usec && cpu.registers.getflag(C) {
        1
    } else {
        0
    };
    let a = cpu.registers.a;
    let r = a.wrapping_add(b).wrapping_add(c);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (b & 0xF) + c > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers
        .flag(C, (a as u16) + (b as u16) + (c as u16) > 0xFF);
    cpu.registers.a = r;
}

fn alu_add16(cpu: &mut Cpu, b: u16) {
    let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
    let r = a.wrapping_add(b);
    cpu.registers.flag(H, (a & 0x07FF) + (b & 0x07FF) > 0x07FF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, a > 0xFFFF - b);
    cpu.registers.h = (r >> 8) as u8;
    cpu.registers.l = (r & 0x00FF) as u8;
}

pub fn alu_add16imm(cpu: &mut Cpu) -> u16 {
    let a = cpu.registers.sp;
    let b = cpu.get_byte() as i8 as i16 as u16;
    cpu.registers.flag(N, false);
    cpu.registers.flag(Z, false);
    cpu.registers.flag(H, (a & 0x000F) + (b & 0x000F) > 0x000F);
    cpu.registers.flag(C, (a & 0x00FF) + (b & 0x00FF) > 0x00FF);
    return a.wrapping_add(b);
}

pub fn alu_or(cpu: &mut Cpu, b: u8) {
    let r = cpu.registers.a | b;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}

pub fn addr_b(cpu: &mut Cpu) {
    let b = cpu.registers.b;
    let a = cpu.registers.a;
    let r = a.wrapping_add(b);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (b & 0xF) > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, (a as u16) + (b as u16) > 0xFF);
    cpu.registers.a = r;
}
pub fn addr_c(cpu: &mut Cpu) {
    let b = cpu.registers.c;
    let a = cpu.registers.a;
    let r = a.wrapping_add(b);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (b & 0xF) > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, (a as u16) + (b as u16) > 0xFF);
    cpu.registers.a = r;
}
pub fn addr_d(cpu: &mut Cpu) {
    let b = cpu.registers.d;
    let a = cpu.registers.a;
    let r = a.wrapping_add(b);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (b & 0xF) > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, (a as u16) + (b as u16) > 0xFF);
    cpu.registers.a = r;
}
pub fn addr_e(cpu: &mut Cpu) {
    let e = cpu.registers.e;
    let a = cpu.registers.a;
    let r = a.wrapping_add(e);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (e & 0xF) > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, (a as u16) + (e as u16) > 0xFF);
    cpu.registers.a = r;
}
pub fn addr_h(cpu: &mut Cpu) {
    let e = cpu.registers.h;
    let a = cpu.registers.a;
    let r = a.wrapping_add(e);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (e & 0xF) > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, (a as u16) + (e as u16) > 0xFF);
    cpu.registers.a = r;
}
pub fn addr_l(cpu: &mut Cpu) {
    let e = cpu.registers.l;
    let a = cpu.registers.a;
    let r = a.wrapping_add(e);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (e & 0xF) > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, (a as u16) + (e as u16) > 0xFF);
    cpu.registers.a = r;
}
pub fn addr_a(cpu: &mut Cpu) {
    let e = cpu.registers.a;
    let a = cpu.registers.a;
    let r = a.wrapping_add(e);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (e & 0xF) > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, (a as u16) + (e as u16) > 0xFF);
    cpu.registers.a = r;
}
pub fn addhl(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    let e = cpu.memory.rb(addr);
    let a = cpu.registers.a;
    let r = a.wrapping_add(e);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (e & 0xF) > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, (a as u16) + (e as u16) > 0xFF);
    cpu.registers.a = r;
}
pub fn addn(cpu: &mut Cpu) {
    let b = cpu.get_byte();
    let a = cpu.registers.a;
    let r = a.wrapping_add(b);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (b & 0xF) > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, (a as u16) + (b as u16) > 0xFF);
    cpu.registers.a = r;
}
pub fn addhlbc(cpu: &mut Cpu) {
    let b = ((cpu.registers.b as u16) << 8) | (cpu.registers.c as u16);
    let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
    let r = a.wrapping_add(b);
    cpu.registers.flag(H, (a & 0x07FF) + (b & 0x07FF) > 0x07FF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, a > 0xFFFF - b);

    cpu.registers.h = (r >> 8) as u8;
    cpu.registers.l = (r & 0x00FF) as u8;
}
pub fn addhlde(cpu: &mut Cpu) {
    let de = ((cpu.registers.d as u16) << 8) | (cpu.registers.e as u16);
    let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
    let r = a.wrapping_add(de);
    cpu.registers.flag(H, (a & 0x07FF) + (de & 0x07FF) > 0x07FF);
    cpu.registers.flag(N, false);
    cpu.registers.flag(C, a > 0xFFFF - de);

    cpu.registers.h = (r >> 8) as u8;
    cpu.registers.l = (r & 0x00FF) as u8;
}
pub fn addhlhl(cpu: &mut Cpu) {
    let hl = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
    alu_add16(cpu, hl);
}
pub fn addhlsp(cpu: &mut Cpu) {
    alu_add16(cpu, cpu.registers.sp);
}
// https://github.com/alexcrichton/jba/blob/rust/src/cpu/z80/imp.rs#L81
pub fn addspn(cpu: &mut Cpu) {
    cpu.registers.sp = alu_add16imm(cpu);
}
pub fn adcr_b(cpu: &mut Cpu) {
    alu_add(cpu, cpu.registers.b, true);
}
pub fn adcr_c(cpu: &mut Cpu) {
    alu_add(cpu, cpu.registers.c, true);
}
pub fn adcr_d(cpu: &mut Cpu) {
    alu_add(cpu, cpu.registers.d, true);
}
pub fn adcr_e(cpu: &mut Cpu) {
    alu_add(cpu, cpu.registers.e, true);
}
pub fn adcr_h(cpu: &mut Cpu) {
    alu_add(cpu, cpu.registers.h, true);
}
pub fn adcr_l(cpu: &mut Cpu) {
    alu_add(cpu, cpu.registers.l, true);
}
pub fn adcr_a(cpu: &mut Cpu) {
    alu_add(cpu, cpu.registers.a, true);
}
pub fn adchl(cpu: &mut Cpu) {
    let hl = cpu
        .memory
        .rb(((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16));
    alu_add(cpu, hl, true);
}
pub fn adcn(cpu: &mut Cpu) {
    let b = cpu.get_byte();
    let c = if cpu.registers.getflag(C) { 1 } else { 0 };
    let a = cpu.registers.a;
    let r = a.wrapping_add(b).wrapping_add(c);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0xF) + (b & 0xF) + c > 0xF);
    cpu.registers.flag(N, false);
    cpu.registers
        .flag(C, (a as u16) + (b as u16) + (c as u16) > 0xFF);
    cpu.registers.a = r;
}
pub fn subr_b(cpu: &mut Cpu) {
    let b = cpu.registers.b;
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16));
    cpu.registers.a = r;
}
pub fn subr_c(cpu: &mut Cpu) {
    let b = cpu.registers.c;
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16));
    cpu.registers.a = r;
}
pub fn subr_d(cpu: &mut Cpu) {
    let b = cpu.registers.d;
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16));
    cpu.registers.a = r;
}
pub fn subr_e(cpu: &mut Cpu) {
    let b = cpu.registers.e;
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16));
    cpu.registers.a = r;
}
pub fn subr_h(cpu: &mut Cpu) {
    let b = cpu.registers.h;
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16));
    cpu.registers.a = r;
}
pub fn subr_l(cpu: &mut Cpu) {
    let b = cpu.registers.l;
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16));
    cpu.registers.a = r;
}
pub fn subr_a(cpu: &mut Cpu) {
    let b = cpu.registers.a;
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(0);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16));
    cpu.registers.a = r;
}
pub fn subhl(cpu: &mut Cpu) {
    let v = cpu
        .memory
        .rb(((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16));
    alu_sub(cpu, v, false);
}
pub fn subn(cpu: &mut Cpu) {
    let b = cpu.get_byte();
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16));
    cpu.registers.a = r;
}
pub fn sbcr_b(cpu: &mut Cpu) {
    alu_sub(cpu, cpu.registers.b, true);
}
pub fn sbcr_c(cpu: &mut Cpu) {
    alu_sub(cpu, cpu.registers.c, true);
}
pub fn sbcr_d(cpu: &mut Cpu) {
    alu_sub(cpu, cpu.registers.d, true);
}
pub fn sbcr_e(cpu: &mut Cpu) {
    alu_sub(cpu, cpu.registers.e, true);
}
pub fn sbcr_h(cpu: &mut Cpu) {
    alu_sub(cpu, cpu.registers.h, true);
}
pub fn sbcr_l(cpu: &mut Cpu) {
    alu_sub(cpu, cpu.registers.l, true);
}
pub fn sbcr_a(cpu: &mut Cpu) {
    alu_sub(cpu, cpu.registers.a, true);
}
pub fn sbchl(cpu: &mut Cpu) {
    let hl = cpu
        .memory
        .rb(((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16));
    alu_sub(cpu, hl, true);
}
pub fn sbcn(cpu: &mut Cpu) {
    let b = cpu.get_byte();
    let c = if cpu.registers.getflag(C) { 1 } else { 0 };
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b).wrapping_sub(c);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F) + c);
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16) + (c as u16));
    cpu.registers.a = r;
}
pub fn cpr_b(cpu: &mut Cpu) {
    let v = cpu.registers.b;
    let r = cpu.registers.a;
    alu_sub(cpu, v, false);
    cpu.registers.a = r;
}
pub fn cpr_c(cpu: &mut Cpu) {
    let v = cpu.registers.c;
    let r = cpu.registers.a;
    alu_sub(cpu, v, false);
    cpu.registers.a = r;
}
pub fn cpr_d(cpu: &mut Cpu) {
    let v = cpu.registers.d;
    let r = cpu.registers.a;
    alu_sub(cpu, v, false);
    cpu.registers.a = r;
}
pub fn cpr_e(cpu: &mut Cpu) {
    let v = cpu.registers.e;
    let r = cpu.registers.a;
    alu_sub(cpu, v, false);
    cpu.registers.a = r;
}
pub fn cpr_h(cpu: &mut Cpu) {
    let v = cpu.registers.h;
    let r = cpu.registers.a;
    alu_sub(cpu, v, false);
    cpu.registers.a = r;
}
pub fn cpr_l(cpu: &mut Cpu) {
    let v = cpu.registers.l;
    let r = cpu.registers.a;
    alu_sub(cpu, v, false);
    cpu.registers.a = r;
}
pub fn cpr_a(cpu: &mut Cpu) {
    let r = cpu.registers.a;
    let b = cpu.registers.a;
    alu_sub(cpu, b, false);
    cpu.registers.a = r;
}
pub fn cphl(cpu: &mut Cpu) -> u32 {
    let v = cpu
        .memory
        .rb(((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16));
    let r = cpu.registers.a;
    alu_sub(cpu, v, false);
    cpu.registers.a = r;
    2
}
pub fn cpn(cpu: &mut Cpu) {
    let b = cpu.get_byte();
    let a = cpu.registers.a;
    let r = a.wrapping_sub(b);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16));
}
pub fn andr_b(cpu: &mut Cpu) {
    let b = cpu.registers.b;
    let r = cpu.registers.a & b;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, true);
    cpu.registers.flag(C, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn andr_c(cpu: &mut Cpu) {
    let c = cpu.registers.c;
    let r = cpu.registers.a & c;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, true);
    cpu.registers.flag(C, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn andr_d(cpu: &mut Cpu) {
    let d = cpu.registers.d;
    let r = cpu.registers.a & d;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, true);
    cpu.registers.flag(C, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn andr_e(cpu: &mut Cpu) {
    let e = cpu.registers.e;
    let r = cpu.registers.a & e;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, true);
    cpu.registers.flag(C, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn andr_h(cpu: &mut Cpu) {
    let h = cpu.registers.h;
    let r = cpu.registers.a & h;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, true);
    cpu.registers.flag(C, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn andr_l(cpu: &mut Cpu) {
    let l = cpu.registers.l;
    let r = cpu.registers.a & l;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, true);
    cpu.registers.flag(C, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn andr_a(cpu: &mut Cpu) {
    alu_and(cpu, cpu.registers.a);
}
pub fn andhl(cpu: &mut Cpu) {
    let hl = cpu
        .memory
        .rb(((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16));
    alu_and(cpu, hl);
}
pub fn andn(cpu: &mut Cpu) {
    let v = cpu.get_byte();
    let r = cpu.registers.a & v;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, true);
    cpu.registers.flag(C, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn orr_b(cpu: &mut Cpu) {
    let r = cpu.registers.a | cpu.registers.b;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn orr_c(cpu: &mut Cpu) {
    let r = cpu.registers.a | cpu.registers.c;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn orr_d(cpu: &mut Cpu) {
    let r = cpu.registers.a | cpu.registers.d;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn orr_e(cpu: &mut Cpu) {
    let r = cpu.registers.a | cpu.registers.e;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn orr_h(cpu: &mut Cpu) {
    let r = cpu.registers.a | cpu.registers.h;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn orr_l(cpu: &mut Cpu) {
    let r = cpu.registers.a | cpu.registers.l;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn orr_a(cpu: &mut Cpu) {
    let r = cpu.registers.a | cpu.registers.a;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn orhl(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + (cpu.registers.l as u16);
    let v = cpu.memory.rb(addr);
    let r = cpu.registers.a | v;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn orn(cpu: &mut Cpu) {
    let b = cpu.get_byte();
    let r = cpu.registers.a ^ b;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn xorr_b(cpu: &mut Cpu) {
    let r = cpu.registers.a ^ cpu.registers.b;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn xorr_c(cpu: &mut Cpu) {
    let r = cpu.registers.a ^ cpu.registers.c;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn xorr_d(cpu: &mut Cpu) {
    let r = cpu.registers.a ^ cpu.registers.d;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn xorr_e(cpu: &mut Cpu) {
    let r = cpu.registers.a ^ cpu.registers.e;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn xorr_h(cpu: &mut Cpu) {
    let r = cpu.registers.a ^ cpu.registers.h;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn xorr_l(cpu: &mut Cpu) {
    let r = cpu.registers.a ^ cpu.registers.l;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn xorr_a(cpu: &mut Cpu) {
    let r = cpu.registers.a ^ cpu.registers.a;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn xorhl(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + (cpu.registers.l as u16);
    let v = cpu.memory.rb(addr);
    let r = cpu.registers.a ^ v;
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(C, false);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn incr_c(cpu: &mut Cpu) {
    let addr = cpu.registers.c;
    let r = addr.wrapping_add(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (addr & 0x0F) + 1 > 0x0F);
    cpu.registers.flag(N, false);
    cpu.registers.c = r;
}
pub fn incr_d(cpu: &mut Cpu) {
    let addr = cpu.registers.d;
    let r = addr.wrapping_add(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (addr & 0x0F) + 1 > 0x0F);
    cpu.registers.flag(N, false);
    cpu.registers.d = r;
}
pub fn incr_e(cpu: &mut Cpu) {
    let addr = cpu.registers.e;
    let r = addr.wrapping_add(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (addr & 0x0F) + 1 > 0x0F);
    cpu.registers.flag(N, false);
    cpu.registers.e = r;
}
pub fn incr_h(cpu: &mut Cpu) {
    let addr = cpu.registers.h;
    let r = addr.wrapping_add(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (addr & 0x0F) + 1 > 0x0F);
    cpu.registers.flag(N, false);
    cpu.registers.h = r;
}
pub fn incr_l(cpu: &mut Cpu) {
    let addr = cpu.registers.l;
    let r = addr.wrapping_add(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (addr & 0x0F) + 1 > 0x0F);
    cpu.registers.flag(N, false);
    cpu.registers.l = r;
}
pub fn incr_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let r = a.wrapping_add(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) + 1 > 0x0F);
    cpu.registers.flag(N, false);
    cpu.registers.a = r;
}
pub fn incr_b(cpu: &mut Cpu) {
    let b = cpu.registers.b;
    let r = b.wrapping_add(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (b & 0x0F) + 1 > 0x0F);
    cpu.registers.flag(N, false);
    cpu.registers.b = r;
}
pub fn inchlm(cpu: &mut Cpu) {
    let a = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
    let v = cpu.memory.rb(a);
    let r = v.wrapping_add(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (v & 0x0F) + 1 > 0x0F);
    cpu.registers.flag(N, false);
    cpu.memory.wb(a, r);
}
pub fn decr_b(cpu: &mut Cpu) {
    let a = cpu.registers.b;
    let r = a.wrapping_sub(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) == 0);
    cpu.registers.flag(N, true);
    cpu.registers.b = r;
}
pub fn decr_c(cpu: &mut Cpu) {
    let c = cpu.registers.c;
    let r = c.wrapping_sub(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (c & 0x0F) == 0);
    cpu.registers.flag(N, true);
    cpu.registers.c = r;
}
pub fn decr_e(cpu: &mut Cpu) {
    let e = cpu.registers.e;
    let r = e.wrapping_sub(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (e & 0x0F) == 0);
    cpu.registers.flag(N, true);
    cpu.registers.e = r;
}
pub fn decr_d(cpu: &mut Cpu) {
    let d = cpu.registers.d;
    let r = d.wrapping_sub(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (d & 0x0F) == 0);
    cpu.registers.flag(N, true);
    cpu.registers.d = r;
}
pub fn decr_h(cpu: &mut Cpu) {
    let h = cpu.registers.h;
    let r = h.wrapping_sub(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (h & 0x0F) == 0);
    cpu.registers.flag(N, true);
    cpu.registers.h = r;
}
pub fn decr_l(cpu: &mut Cpu) {
    let l = cpu.registers.l;
    let r = l.wrapping_sub(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (l & 0x0F) == 0);
    cpu.registers.flag(N, true);
    cpu.registers.l = r;
}
pub fn decr_a(cpu: &mut Cpu) {
    let a = cpu.registers.a;
    let r = a.wrapping_sub(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) == 0);
    cpu.registers.flag(N, true);
    cpu.registers.a = r;
}
pub fn dechlm(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
    let v = cpu.memory.rb(addr);
    let r = v.wrapping_sub(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (v & 0x0F) == 0);
    cpu.registers.flag(N, true);
    cpu.memory.wb(addr, r);
}
pub fn incbc(cpu: &mut Cpu) {
    let mut val = ((cpu.registers.b as u16) << 8) | (cpu.registers.c as u16);
    val = val.wrapping_add(1);
    cpu.registers.b = (val >> 8) as u8;
    cpu.registers.c = (val & 0x00FF) as u8;
}
pub fn incde(cpu: &mut Cpu) {
    let mut val = ((cpu.registers.d as u16) << 8) | (cpu.registers.e as u16);
    val = val.wrapping_add(1);
    cpu.registers.d = (val >> 8) as u8;
    cpu.registers.e = (val & 0x00FF) as u8;
}
pub fn inchl(cpu: &mut Cpu) {
    let mut val = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
    val = val.wrapping_add(1);
    cpu.registers.h = (val >> 8) as u8;
    cpu.registers.l = (val & 0x00FF) as u8;
}
pub fn set_hl(cpu: &mut Cpu, val: u16) {
    cpu.registers.h = (val >> 8) as u8;
    cpu.registers.l = (val & 0x00FF) as u8;
}
pub fn incsp(cpu: &mut Cpu) {
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
}
pub fn decbc(cpu: &mut Cpu) {
    let mut val = ((cpu.registers.b as u16) << 8) | (cpu.registers.c as u16);
    val = val.wrapping_sub(1);
    cpu.registers.b = (val >> 8) as u8;
    cpu.registers.c = (val & 0x00FF) as u8;
}
pub fn decde(cpu: &mut Cpu) {
    let val =
        (((cpu.registers.d as u16) << 8) | (cpu.registers.e as u16)).wrapping_sub(1);
    cpu.registers.d = (val >> 8) as u8;
    cpu.registers.e = (val & 0x00FF) as u8;
}
pub fn dechl(cpu: &mut Cpu) {
    let v = ((cpu.registers.h as u16) << 8) | (cpu.registers.l as u16);
    let value = v.wrapping_sub(1);
    cpu.registers.h = (value >> 8) as u8;
    cpu.registers.l = (value & 0x00FF) as u8;
}
pub fn decsp(cpu: &mut Cpu) {
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
}
pub fn cpl(cpu: &mut Cpu) {
    cpu.registers.a = !cpu.registers.a;
    cpu.registers.flag(H, true);
    cpu.registers.flag(N, true);
}
pub fn scf(cpu: &mut Cpu) {
    cpu.registers.flag(C, true);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
}
pub fn ccf(cpu: &mut Cpu) {
    let v = !cpu.registers.getflag(C);
    cpu.registers.flag(C, v);
    cpu.registers.flag(H, false);
    cpu.registers.flag(N, false);
}
