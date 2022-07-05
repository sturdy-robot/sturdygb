#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Copy, Clone)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | ((self.f & 0xF0) as u16)
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | ((self.c) as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | ((self.e) as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | ((self.l) as u16)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[derive(Copy, Clone)]
pub enum CPUFlags {
    C = 0b00010000,
    N = 0b00100000,
    H = 0b01000000,
    Z = 0b10000000,
}

#[cfg(test)]
mod test {
    use super::Registers;
    use super::CPUFlags;

    #[test]
    fn test_new_registers() {
        let mut r: Registers = Registers::new();
        assert_eq!(r.a, 0x01);
        assert_eq!(r.b, 0x00);
        assert_eq!(r.c, 0x13);
        assert_eq!(r.d, 0x00);
        assert_eq!(r.e, 0xD8);
        assert_eq!(r.f, 0xB0);
        assert_eq!(r.h, 0x01);
        assert_eq!(r.l, 0x4D);
        assert_eq!(r.sp, 0xFFFE);
        assert_eq!(r.pc, 0x0100);
    }

    #[test]
    fn test_set_registers() {
        let mut r: Registers = Registers::new();
        r.a = 0xAA;
        r.b = 0xBB;
        r.c = 0x33;
        r.d = 0x55;
        r.e = 0x11;
        r.f = 0xF0;
        r.h = 0x13;
        r.l = 0x14;
        r.sp = 0x1234;
        r.pc = 0x4444;
        assert_eq!(r.a, 0xAA);
        assert_eq!(r.b, 0xBB);
        assert_eq!(r.c, 0x33);
        assert_eq!(r.d, 0x55);
        assert_eq!(r.e, 0x11);
        assert_eq!(r.f, 0xF0);
        assert_eq!(r.h, 0x13);
        assert_eq!(r.l, 0x14);
        assert_eq!(r.sp, 0x1234);
        assert_eq!(r.pc, 0x4444);

        assert_eq!(r.af(), 0xAAF0);
        assert_eq!(r.bc(), 0xBB33);
        assert_eq!(r.de(), 0x5511);
        assert_eq!(r.hl(), 0x1314);
    }

    #[test]
    fn test_set_wide_registers() {
        let mut r: Registers = Registers::new();
        r.set_af(0x1111);
        r.set_bc(0x2222);
        r.set_de(0x3333);
        r.set_hl(0x4444);
        assert_eq!(r.af(), 0x1110);
        assert_eq!(r.bc(), 0x2222);
        assert_eq!(r.de(), 0x3333);
        assert_eq!(r.hl(), 0x4444);
    }
}