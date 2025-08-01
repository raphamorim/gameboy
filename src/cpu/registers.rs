use crate::mode::GbMode;
#[cfg(not(feature = "ffi"))]
use std::fmt;
#[cfg(feature = "ffi")]
use core::fmt;

#[derive(Debug)]
pub struct Registers {
    // 8-bit registers
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    // The flags register (F)
    // it automatically calculates certain bits, or flags, based on the result of the last operation.
    pub f: u8,

    // 16-bit registers
    pub pc: u16,
    pub sp: u16,
}

#[derive(Copy, Clone)]
pub enum CpuFlag {
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "a:{:2x} f:{:2x} b:{:2x} c:{:2x} d:{:2x} e:{:2x} \
                h:{:2x} l:{:2x} pc:{:4x} sp:{:4x}",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
            self.f,
            self.h,
            self.l,
            self.pc,
            self.sp
        )
    }
}

impl Registers {
    pub fn new(mode: GbMode) -> Registers {
        use CpuFlag::*;
        match mode {
            GbMode::Classic => Registers {
                a: 0x01,
                f: C as u8 | H as u8 | Z as u8,
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                pc: 0x0100,
                sp: 0xFFFE,
            },
            GbMode::ColorAsClassic => Registers {
                a: 0x11,
                f: Z as u8,
                b: 0x00,
                c: 0x00,
                d: 0x00,
                e: 0x08,
                h: 0x00,
                l: 0x7C,
                pc: 0x0100,
                sp: 0xFFFE,
            },
            GbMode::Color => Registers {
                a: 0x11,
                f: Z as u8,
                b: 0x00,
                c: 0x00,
                d: 0xFF,
                e: 0x56,
                h: 0x00,
                l: 0x0D,
                pc: 0x0100,
                sp: 0xFFFE,
            },
        }
    }

    pub fn flag(&mut self, flags: CpuFlag, set: bool) {
        let mask = flags as u8;
        match set {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
        self.f &= 0xF0;
    }

    pub fn getflag(&self, flags: CpuFlag) -> bool {
        let mask = flags as u8;
        self.f & mask > 0
    }
}
