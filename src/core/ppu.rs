const FRAME_WIDTH: u16 = 256;
const FRAME_HEIGHT: u16 = 256;

const GB_WIDTH: u8 = 160;
const GB_HEIGHT: u8 = 144;

pub(crate) struct Ppu {
    pub(crate) vram: [u8; 0x2000],
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    dma: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    bcps: u8,
    bcpd: u8,
    ocps: u8,
    ocpd: u8,
}

impl Ppu {
    pub(crate) fn new() -> Self {
        Self {
            vram: [0; 0x2000],
            lcdc: 0x91,
            stat: 0x81,
            scy: 0x00,
            scx: 0x00,
            ly: 0x91,
            lyc: 0x00,
            dma: 0xFF,
            bgp: 0xFC,
            obp0: 0x00,
            obp1: 0x00,
            wy: 0x00,
            wx: 0x00,
            bcps: 0xFF,
            bcpd: 0xFF,
            ocps: 0xFF,
            ocpd: 0xFF,
        }
    }

    pub(crate) fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.bgp,
            0xFF47 => self.obp0,
            0xFF48 => self.obp1,
            0xFF49 => self.wy,
            0xFF4A => self.wx,
            0xFF68 => self.bcps,
            0xFF69 => self.bcpd,
            0xFF6A => self.ocps,
            0xFF6B => self.ocpd,
            _ => panic!("Invalid memory address!"),
        }
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => self.ly = value,
            0xFF45 => self.lyc = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            0xFF68 => self.bcps = value,
            0xFF69 => self.bcpd = value,
            0xFF6A => self.ocps = value,
            0xFF6B => self.ocpd = value,
            _ => panic!("Invalid memory address!"),
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_ppu() -> Ppu {
        Ppu::new()
    }

    #[test]
    fn test() {
        
    }
}