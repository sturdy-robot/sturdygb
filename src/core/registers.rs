#[derive(Copy, Clone, PartialEq, Eq)]
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
    pub ime: bool,
}

impl Registers {
    pub fn new(a: u8, f: u8, b: u8, c: u8, d: u8, e: u8, h: u8, l: u8) -> Self {
        Self { a, f, b, c, d, e, h, l, sp: 0xFFFE, pc: 0x0100, ime: true }
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

    pub fn set_f(&mut self, mut c: u8, mut h: u8, mut n: u8, mut z: u8) {
        if c == 2 {
            c = self.f & FFlags::C as u8;
        }
        if h == 2 {
            h = self.f & FFlags::H as u8;
        }
        if n == 2 {
            n = self.f & FFlags::N as u8;
        }
        if z == 2 {
            z = self.f & FFlags::Z as u8;
        }
        let value: u8 = ((z << 8) | (n << 7) | (h << 6) | (c << 5)) & 0xF0;

    }

    pub fn get_flag(&self, flag: FFlags) -> u8 {
        match self.f & (flag as u8) > 0 {
            true => 1,
            false => 0,
        }
    }
}

#[derive(Copy, Clone)]
pub enum FFlags {
    C = 0x10,
    N = 0x40,
    H = 0x20,
    Z = 0x80,
}

#[cfg(test)]
mod test {
    use super::Registers;

    fn get_registers() -> Registers {
        Registers::new(0x01, 0x00, 0xFF, 0x13, 0x00, 0xC1, 0x84, 0x03)
    }


    #[test]
    fn test_new_registers() {
        let r: Registers = get_registers();
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
        let mut r = get_registers();
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
        let mut r: Registers = get_registers();
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
