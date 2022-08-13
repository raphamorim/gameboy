use std::fmt;

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
            self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l, self.pc, self.sp
        )
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0xb0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            pc: 0x0100,
            sp: 0xFFFE,
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_clock_set() {
//         let mut clock: Clock = Clock { t: 0, m: 0 };
//         assert_eq!(clock.t, 0);
//         assert_eq!(clock.m, 0);

//         clock.set_m(1);
//         clock.set_t(2);

//         assert_eq!(clock.t, 2);
//         assert_eq!(clock.m, 1);
//     }
// }
