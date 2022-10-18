pub struct IO {
    joypad: Joypad,
    timer: Timer,
    serial: Serial,
    sound: Sound,
}

impl IO {
    pub fn new() -> Self {
        Self {
            joypad: Joypad::new(),
            timer: Timer::new(),
            serial: Serial::new(),
            sound: Sound::new(),
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad.read_byte(address),
            0xFF01 ..= 0xFF02 => self.serial.read_byte(address),
            0xFF04 ..= 0xFF07 => self.timer.read_byte(address),
            0xFF10 ..= 0xFF26 => self.sound.read_byte(address),
            _ => 0xFF, // TODO: implement this
        }
    }
}
pub struct Joypad {

}

impl Joypad {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        // TODO: IMPLEMENT THIS
        0x00
    }
}

pub struct Timer {

}

impl Timer {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        // TODO: IMPLEMENT THIS
        0x00
    }
}

pub struct Serial {
    
}

impl Serial {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        // TODO: IMPLEMENT THIS
        0x00
    }
}

pub struct Sound {

}

impl Sound {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        // TODO: IMPLEMENT THIS
        0x00
    }
}

