use rand::RngCore;

const FRAME_WIDTH: u16 = 256;
const FRAME_HEIGHT: u16 = 256;

const GB_WIDTH: u8 = 160;
const GB_HEIGHT: u8 = 144;

pub struct Ppu {
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
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
    vbk: usize,
    hdma1: u8,
    hdma2: u8,
    hdma3: u8,
    hdma4: u8,
    hdma5: u8,
    bcps: u8,
    bcpd: u8,
    ocps: u8,
    ocpd: u8,
    clock: u32,
}

impl Ppu {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut vram = [0_u8; 0x2000];
        let mut oam = [0_u8; 0xA0];
        rng.fill_bytes(&mut vram);
        rng.fill_bytes(&mut oam);

        Self {
            vram: vram,
            oam: oam,
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
            vbk: 0x00,
            hdma1: 0xFF,
            hdma2: 0xFF,
            hdma3: 0xFF,
            hdma4: 0xFF,
            hdma5: 0xFF,
            bcps: 0xFF,
            bcpd: 0xFF,
            ocps: 0xFF,
            ocpd: 0xFF,
            clock: 0,
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
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
            0xFF4F => self.vbk as u8,
            0xFF51 => self.hdma1,
            0xFF52 => self.hdma2,
            0xFF53 => self.hdma3,
            0xFF54 => self.hdma4,
            0xFF55 => self.hdma5,
            0xFF68 => self.bcps,
            0xFF69 => self.bcpd,
            0xFF6A => self.ocps,
            0xFF6B => self.ocpd,
            _ => 0xFF
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF45 => self.lyc = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            0xFF4F => self.vbk = value as usize,
            0xFF51 => self.hdma1 = value,
            0xFF52 => self.hdma2 = value,
            0xFF53 => self.hdma3 = value,
            0xFF54 => self.hdma4 = value,
            0xFF55 => self.hdma5 = value,
            0xFF68 => self.bcps = value,
            0xFF69 => self.bcpd = value,
            0xFF6A => self.ocps = value,
            0xFF6B => self.ocpd = value,
            _ => {
                println!("Write to invalid memory address: {address:04X}");
            },
        };
    }
    
    pub fn get_mode(&mut self) -> u8 {
        match self.stat & 0x03 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            _ => unreachable!(),
        }
    }

    fn set_mode(&mut self, mode: u8) {
        match mode {
            0 => self.stat = self.stat & 0x7C,
            1 => self.stat = self.stat & 0x7D,
            2 => self.stat = self.stat & 0x7E,
            3 => self.stat = self.stat & 0x7F,
            _ => unreachable!(),
        };
    }
}


impl Ppu {
    pub fn execute(&mut self) {
        match self.get_mode() {
            0 => {
                if self.clock >= 204 {
                    self.clock = 0;
                    self.ly += 1;

                    if self.ly == 143 {
                        self.set_mode(1);

                        // self.put_image_data()

                    } else {
                        self.set_mode(2);
                    }
                    
                }
            },
            1 => {
                if self.clock >= 456 {
                    self.clock = 0;
                    self.ly += 1;

                    if self.ly > 153 {
                        self.set_mode(2);
                        self.ly = 0;
                    }
                }

            },
            2 => {
                if self.clock >= 80 {
                    self.clock = 0;
                    self.set_mode(3);
                }
            },
            3 => {
                if self.clock >= 172 {
                    self.clock = 0;
                    self.set_mode(0);
                    self.render_scan_line();
                }
            },
            _ => unreachable!(),
        }
    }

    fn render_scan_line(&mut self) {

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