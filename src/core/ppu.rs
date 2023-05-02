use super::mbc::GbMode;
use super::Memory;
use super::gb::Gb;

pub enum PpuMode {
    Hblank,
    Vblank,
    SearchingOAM,
    Transferring,
}


pub struct Ppu {
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
    key1: u8,
    vbk: u8,
    bcps: u8,
    bcpd: u8,
    ocps: u8,
    ocpd: u8,
    svbk: u8,
    pub vram: Vec<u8>,
    pub oam: [u8; 0xA0],
    pub oam_corruption_bug: bool,
    pub cycles: u32,
}

impl Ppu {
    pub fn new(gb_mode: &GbMode) -> Self {
        let vram: Vec<u8>;
        match gb_mode {
            GbMode::CgbMode => vram = vec![0; 0x4000],
            _ => vram = vec![0; 0x2000],
        }
        let oam: [u8; 0xA0] = [0; 0xA0];
        Self {
            lcdc: 0x91,
            stat: 0x81,
            scy: 0,
            scx: 0,
            ly: 0x91,
            lyc: 0,
            dma: 0xFF,
            bgp: 0xFC,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            key1: 0xFF,
            vbk: 0xFF,
            bcps: 0xFF,
            bcpd: 0xFF,
            ocps: 0xFF,
            ocpd: 0xFF,
            svbk: 0xFF,
            vram,
            oam,
            oam_corruption_bug: false,
            cycles: 0,
        }
    }
}

impl Memory for Ppu {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => {
                if self.vram.len() == 0x4000 {
                    self.vram[((self.vbk as usize & 1) * 0x2000) | ((address & 0x1FFF) as usize)]
                } else {
                    self.vram[(address & 0x1FFF) as usize]
                }
            }
            0xFE00..=0xFE9F => todo!(),
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF4D => self.key1,
            0xFF4F => self.vbk,
            0xFF68 => self.bcps,
            0xFF69 => self.bcpd,
            0xFF6A => self.ocps,
            0xFF6B => self.ocpd,
            0xFF70 => self.svbk,
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9FFF => match self.stat & 0x03 {
                0..=2 => {
                    if self.vram.len() == 0x4000 {
                        self.vram
                            [((self.vbk as usize & 1) * 0x2000) | ((address & 0x1FFF) as usize)] =
                            value;
                    } else {
                        self.vram[(address & 0x1FFF) as usize] = value;
                    }
                }
                _ => { },
            },
            0xFE00..=0xFE9F => match self.stat & 0x03 {
                0 | 1 => self.oam[(address & 0x9F) as usize] = value,
                _ => { },
            },
            0xFF40 => self.lcdc = value,
            0xFF41 => self.stat = value,
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => self.ly = value,
            0xFF45 => self.lyc = value,
            0xFF46 => self.dma = value,
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            0xFF4D => self.key1 = value,
            0xFF4F => self.vbk = value,
            0xFF68 => self.bcps = value,
            0xFF69 => self.bcpd = value,
            0xFF6A => self.ocps = value,
            0xFF6B => self.ocpd = value,
            0xFF70 => self.svbk = value,
            _ => {}
        };
    }
}

impl Ppu {
    pub fn run(&self) {
        let gpu_mode = self.get_gpu_mode();
        match gpu_mode {
            PpuMode::Hblank => {

            }
            PpuMode::Vblank => {

            }
            PpuMode::SearchingOAM => {

            }
            PpuMode::Transferring => {

            }
        }

    }

    fn check_stat_interrupt(&self) -> bool {
        todo!()
    }

    fn get_gpu_mode(&self) -> PpuMode {
        match self.stat & 0x03 {
            0 => PpuMode::Hblank,
            1 => PpuMode::Vblank,
            2 => PpuMode::SearchingOAM,
            3 => PpuMode::Transferring,
            _ => unreachable!()
        }
    }
}

impl Gb {
    
}