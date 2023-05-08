use crate::cpu::cpu::Cpu;
use crate::cpu::registers::CpuFlag::{C, H, N, Z};

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
    panic!("fix f attribution");
    let mut hl: u16 = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    hl += ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;

    if hl > 65535 {
        cpu.registers.f |= 0x10;
    } else {
        cpu.registers.f &= 0xEF;
        cpu.registers.h = ((hl >> 8) & 255) as u8;
        cpu.registers.l = (hl & 255) as u8;
    }
}
pub fn addhlsp(cpu: &mut Cpu) {
    panic!("fix f attribution");
    let mut hl = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    hl += cpu.registers.sp;
    if hl > 65535 {
        cpu.registers.f |= 0x10;
    } else {
        cpu.registers.f &= 0xEF;
    }
    cpu.registers.h = ((hl >> 8) & 255) as u8;
    cpu.registers.l = (hl & 255) as u8;
}
// https://github.com/alexcrichton/jba/blob/rust/src/cpu/z80/imp.rs#L81
pub fn addspn(cpu: &mut Cpu) {
    let b = cpu.memory.rb(cpu.registers.pc) as i8 as i16 as u16;
    let res = cpu.registers.sp + b;
    let tmp = b ^ res ^ cpu.registers.sp;
    cpu.registers.f =
        if tmp & 0x100 != 0 { 0x10 } else { 0 } | if tmp & 0x010 != 0 { 0x20 } else { 0 };
    cpu.registers.sp = res;
}
pub fn adcr_b(cpu: &mut Cpu) {
    cpu.registers.a += cpu.registers.b;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a += 1
    } else {
        cpu.registers.a += 0
    }
    cpu.fz(cpu.registers.a, 0);
    if cpu.registers.a > u8::MAX {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn adcr_c(cpu: &mut Cpu) {
    cpu.registers.a += cpu.registers.c;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a += 1
    } else {
        cpu.registers.a += 0
    }
    cpu.fz(cpu.registers.a, 0);
    if cpu.registers.a > u8::MAX {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn adcr_d(cpu: &mut Cpu) {
    cpu.registers.a += cpu.registers.d;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a += 1
    } else {
        cpu.registers.a += 0
    }
    cpu.fz(cpu.registers.a, 0);
    if cpu.registers.a > u8::MAX {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn adcr_e(cpu: &mut Cpu) {
    cpu.registers.a += cpu.registers.e;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a += 1
    } else {
        cpu.registers.a += 0
    }
    cpu.fz(cpu.registers.a, 0);
    if cpu.registers.a > u8::MAX {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn adcr_h(cpu: &mut Cpu) {
    cpu.registers.a += cpu.registers.h;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a += 1
    } else {
        cpu.registers.a += 0
    }
    cpu.fz(cpu.registers.a, 0);
    if cpu.registers.a > u8::MAX {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn adcr_l(cpu: &mut Cpu) {
    cpu.registers.a += cpu.registers.l;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a += 1
    } else {
        cpu.registers.a += 0
    }
    cpu.fz(cpu.registers.a, 0);
    if cpu.registers.a > u8::MAX {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn adcr_a(cpu: &mut Cpu) {
    cpu.registers.a += cpu.registers.a;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a += 1
    } else {
        cpu.registers.a += 0
    }
    cpu.fz(cpu.registers.a, 0);
    if cpu.registers.a > u8::MAX {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn adchl(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.a += cpu.memory.rb(addr);
    if cpu.registers.f >= 0x10 {
        cpu.registers.a += 1
    } else {
        cpu.registers.a += 0
    }
    cpu.fz(cpu.registers.a, 0);
    if cpu.registers.a > u8::MAX {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
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
    cpu.registers.a -= cpu.registers.d;
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn subr_e(cpu: &mut Cpu) {
    cpu.registers.a -= cpu.registers.e;
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn subr_h(cpu: &mut Cpu) {
    cpu.registers.a -= cpu.registers.h;
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn subr_l(cpu: &mut Cpu) {
    cpu.registers.a -= cpu.registers.l;
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
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
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.a -= cpu.memory.rb(addr);
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
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
    cpu.registers.a -= cpu.registers.b;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a -= 1
    } else {
        cpu.registers.a -= 0;
    }
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn sbcr_c(cpu: &mut Cpu) {
    cpu.registers.a -= cpu.registers.c;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a -= 1
    } else {
        cpu.registers.a -= 0;
    }
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn sbcr_d(cpu: &mut Cpu) {
    cpu.registers.a -= cpu.registers.d;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a -= 1
    } else {
        cpu.registers.a -= 0;
    }
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn sbcr_e(cpu: &mut Cpu) {
    cpu.registers.a -= cpu.registers.e;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a -= 1
    } else {
        cpu.registers.a -= 0;
    }
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn sbcr_h(cpu: &mut Cpu) {
    cpu.registers.a -= cpu.registers.h;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a -= 1
    } else {
        cpu.registers.a -= 0;
    }
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn sbcr_l(cpu: &mut Cpu) {
    cpu.registers.a -= cpu.registers.l;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a -= 1
    } else {
        cpu.registers.a -= 0;
    }
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn sbcr_a(cpu: &mut Cpu) {
    cpu.registers.a -= cpu.registers.a;
    if cpu.registers.f >= 0x10 {
        cpu.registers.a -= 1
    } else {
        cpu.registers.a -= 0;
    }
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
}
pub fn sbchl(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.a -= cpu.memory.rb(addr);
    if cpu.registers.f >= 0x10 {
        cpu.registers.a -= 1
    } else {
        cpu.registers.a -= 0;
    }
    cpu.fz(cpu.registers.a, 1);
    if cpu.registers.a < 0 {
        cpu.registers.f |= 0x10;
    }
    cpu.registers.a &= 255;
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
    let mut i = cpu.registers.a;
    i -= cpu.registers.b;
    cpu.fz(i, 1);
    if i < 0 {
        cpu.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_c(cpu: &mut Cpu) {
    let mut i = cpu.registers.a;
    i -= cpu.registers.c;
    cpu.fz(i, 1);
    if i < 0 {
        cpu.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_d(cpu: &mut Cpu) {
    let mut i = cpu.registers.a;
    i -= cpu.registers.d;
    cpu.fz(i, 1);
    if i < 0 {
        cpu.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_e(cpu: &mut Cpu) {
    let mut i = cpu.registers.a;
    i -= cpu.registers.e;
    cpu.fz(i, 1);
    if i < 0 {
        cpu.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_h(cpu: &mut Cpu) {
    let mut i = cpu.registers.a;
    i -= cpu.registers.h;
    cpu.fz(i, 1);
    if i < 0 {
        cpu.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_l(cpu: &mut Cpu) {
    let mut i = cpu.registers.a;
    i -= cpu.registers.l;
    cpu.fz(i, 1);
    if i < 0 {
        cpu.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpr_a(cpu: &mut Cpu) {
    let mut i = cpu.registers.a;
    i -= cpu.registers.a;
    cpu.fz(i, 1);
    if i < 0 {
        cpu.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cphl(cpu: &mut Cpu) {
    let mut i = cpu.registers.a;
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    i -= cpu.memory.rb(addr);
    cpu.fz(i, 1);
    if i < 0 {
        cpu.registers.f |= 0x10;
    }
    i &= 255;
}
pub fn cpn(cpu: &mut Cpu) {
    // let v = cpu.fetchbyte(); cpu.alu_cp(v)

    let b = cpu.get_byte();
    let a = cpu.registers.a;

    // cpu.alu_sub(b, false);
    // cpu.reg.a = r;

    let r = a.wrapping_sub(b);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (a & 0x0F) < (b & 0x0F));
    cpu.registers.flag(N, true);
    cpu.registers.flag(C, (a as u16) < (b as u16));
    // cpu.registers.a = a;

    // let mut i = cpu.registers.a as u16;
    // i = cpu.memory.rw(cpu.registers.pc) - i;
    // cpu.registers.pc += 1;
    // cpu.fz(i as u8, 1);
    // if i < 255 {
    //     cpu.registers.f |= 0x10;
    // }
    // i &= 255;
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
    cpu.registers.a &= cpu.registers.a;
    cpu.registers.a &= 255;
    cpu.fz(cpu.registers.a, 0);
}
pub fn andhl(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.registers.a &= cpu.memory.rb(addr);
    cpu.registers.a &= 255;
    cpu.fz(cpu.registers.a, 0);
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
pub fn xorn(cpu: &mut Cpu) {
    cpu.registers.a ^= cpu.memory.rb(cpu.registers.pc);
    cpu.registers.pc += 1;
    cpu.registers.a &= 255;
    cpu.fz(cpu.registers.a, 0);
}
pub fn incr_b(cpu: &mut Cpu) {
    let addr = cpu.registers.b;
    let r = addr.wrapping_add(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (addr & 0x0F) + 1 > 0x0F);
    cpu.registers.flag(N, false);
    cpu.registers.b = r;
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
pub fn inchlm(cpu: &mut Cpu) {
    let addr = ((cpu.registers.h as u16) << 8) + (cpu.registers.l as u16);
    let mut i = cpu.memory.rb(addr) + 1;
    i &= 255;
    let waddr = ((cpu.registers.h as u16) << 8) + cpu.registers.l as u16;
    cpu.memory.wb(waddr, i);
    cpu.fz(i, 0);
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
    cpu.registers.c -= 1;
    cpu.registers.c &= 255;
    cpu.fz(cpu.registers.c, 0);
}
pub fn decr_d(cpu: &mut Cpu) {
    let d = cpu.registers.d;
    let r = d.wrapping_sub(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (d & 0x0F) == 0);
    cpu.registers.flag(N, true);
    cpu.registers.d = r;
}
pub fn decr_e(cpu: &mut Cpu) {
    let e = cpu.registers.e;
    let r = e.wrapping_sub(1);
    cpu.registers.flag(Z, r == 0);
    cpu.registers.flag(H, (e & 0x0F) == 0);
    cpu.registers.flag(N, true);
    cpu.registers.e = r;
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
pub fn incsp(cpu: &mut Cpu) {
    cpu.registers.sp = cpu.registers.sp + 1;
}
pub fn decbc(cpu: &mut Cpu) {
    let mut val = ((cpu.registers.b as u16) << 8) | (cpu.registers.c as u16);
    val = val.wrapping_sub(1);
    cpu.registers.b = (val >> 8) as u8;
    cpu.registers.c = (val & 0x00FF) as u8;
}
pub fn decde(cpu: &mut Cpu) {
    cpu.registers.e = (cpu.registers.e - 1) & 255;
    if cpu.registers.e == 255 {
        cpu.registers.d = (cpu.registers.d - 1) & 255;
    }
}
pub fn dechl(cpu: &mut Cpu) {
    // cpu.registers.l = (cpu.registers.l - 1) & 255;
    if cpu.registers.l == 255 {
        cpu.registers.h = (cpu.registers.h - 1) & 255;
    }
}
pub fn decsp(cpu: &mut Cpu) {
    cpu.registers.sp = (cpu.registers.sp - 1) & 65535;
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
