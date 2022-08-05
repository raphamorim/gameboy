#[derive(Debug)]
pub struct Clock {
    pub m: u16,
    pub t: u16,
}

impl Clock {
    fn set_t(&mut self, t: u16) {
        self.t = t;
    }

    fn set_m(&mut self, m: u16) {
        self.m = m;
    }
}

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
    pub ime: u8,

    // The flags register (F)
    // it automatically calculates certain bits, or flags, based on the result of the last operation.
    pub f: u8,

    // Clock for last instruction
    pub m: u16,
    pub t: u16,

    // 16-bit registers
    pub pc: u16,
    pub sp: u16,

    // Internal state
    pub clock: Clock,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: 0x01,    // 1
            f: 0xB0,    // 176
            b: 0x00,    // 0
            c: 0x13,    // 19
            d: 0x00,    // 0
            e: 0xD8,    // 216
            h: 0x01,    // 1
            l: 0x4D,    // 77
            pc: 0x0100, // Starts with 256
            sp: 0xFFFE, // 65534
            m: 0,
            t: 0,
            ime: 0,
            clock: Clock { m: 0, t: 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_set() {
        let mut clock: Clock = Clock { t: 0, m: 0 };
        assert_eq!(clock.t, 0);
        assert_eq!(clock.m, 0);

        clock.set_m(1);
        clock.set_t(2);

        assert_eq!(clock.t, 2);
        assert_eq!(clock.m, 1);
    }
}
