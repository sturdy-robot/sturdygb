pub struct IO {
    joypad: Joypad,
    timer: Timer,
    serial: Serial,
    sound: Sound,
    pub ifflag: u8,
}

impl IO {
    pub fn new() -> Self {
        Self {
            joypad: Joypad::new(),
            timer: Timer::new(),
            serial: Serial::new(),
            sound: Sound::new(),
            ifflag: 0,
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad.read_byte(address),
            0xFF01..=0xFF02 => self.serial.read_byte(address),
            0xFF04..=0xFF07 => self.timer.read_byte(address),
            0xFF0F => self.ifflag,
            0xFF10..=0xFF26 => self.sound.read_byte(address),
            _ => 0xFF, // TODO: implement this
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => self.joypad.write_byte(value),
            0xFF01..=0xFF02 => self.serial.write_byte(address, value),
            0xFF04..=0xFF07 => self.timer.write_byte(address, value),
            0xFF0F => self.ifflag = value,
            0xFF10..=0xFF26 => self.sound.write_byte(address, value),
            _ => println!("Writing to invalid memory!"), // TODO: implement this
        };
    }
}
pub struct Joypad {
    memory: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self { memory: 0 }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        // TODO: IMPLEMENT THIS
        0x00
    }

    pub fn write_byte(&mut self, value: u8) {
        self.memory = value;
    }
}

pub struct Timer {
    tima: u8,
    tma: u8,
    tac: u8,
    div: u8,
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
            0xFF04 => self.div,
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

pub struct Serial {
    sb: u8,
    sc: u8,
    memory: [u8; 0x02],
}

impl Serial {
    pub fn new() -> Self {
        Self {
            sb: 0,
            sc: 0,
            memory: [0; 0x02]
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        // TODO: IMPLEMENT THIS
        0x00
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[(address & 0x0E) as usize];
    }
}

pub struct Sound {
    memory: [u8; 0xA0],
}

impl Sound {
    pub fn new() -> Self {
        Self { memory: [0; 0xA0] }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        // TODO: IMPLEMENT THIS
        0x00
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[(address & 0xAF) as usize];
    }
}
