pub struct Timer {
    tima: u8,
    tma: u8,
    tac: u8,
    div: u16,

}

impl Timer {
    pub fn new() -> Self {
        Self {
            tima: 0,
            tma: 0,
            tac: 0,
            div: 0,
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => unreachable!(),
        };
    }
}

impl Timer {
    pub fn execute(&mut self) -> bool {
        let prev_div: u16 = self.div;

        self.div = self.div.wrapping_add(1);

        let mut timer_update: bool = false;
        match self.tac & 0x03 {
            0x00 => timer_update = self.check_div(prev_div & 0x200) && !self.check_div(self.div & (1 << 9)),
            0x01 => timer_update = self.check_div(prev_div & 0x008) && !self.check_div(self.div & (1 << 3)),
            0x02 => timer_update = self.check_div(prev_div & 0x020) && !self.check_div(self.div & (1 << 5)),
            0x03 => timer_update = self.check_div(prev_div & 0x080) && !self.check_div(self.div & (1 << 7)),
            _ => unreachable!(),
        }

        if timer_update && ((self.tac & 0x04) > 0) {
            self.tima = self.tima.wrapping_add(1);

            if self.tima == 0xFF {
                self.tima = self.tma;
            }
        }

        false
    }

    fn check_div(&mut self, prev_div: u16) -> bool {
        prev_div > 0
    }
}