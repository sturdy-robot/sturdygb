use super::cartridge::{Cartridge, MBCTypes};
use super::ppu::{PPU};
use super::io::{IO};

pub struct MMU {
    pub current_rom_bank: u8,
    pub mbc: Cartridge,
    pub ppu: PPU,
    pub io: IO,
    vram: [u8; 0x4000],
    eram: Vec<u8>,
    wram: Vec<u8>,
    oam: Vec<u8>,
    hram: Vec<u8>,
}

impl MMU {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            current_rom_bank: 0,
            mbc: cartridge,
            ppu: PPU::new(),
            io: IO::new(),
            vram: [0; 0x4000],
            eram: Vec::new(),
            wram: Vec::new(),
            oam: Vec::new(),
            hram: Vec::new(),
        }
    }

    pub fn fetch_instruction(&mut self, pc: &mut u16) -> (u8, u16) {
        let instruction = self.read_byte(*pc);
        let inc_pc = pc.wrapping_add(1);
        (instruction, inc_pc)
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x7FFF => self.read_rom(address),
            0x8000 ..= 0x9FFF => self.vram[(address & 0x1FFF) as usize],
            0xA000 ..= 0xBFFF => self.eram[(address & 0x1FFF) as usize],
            0xC000 ..= 0xCFFF | 0xE000 ..= 0xEFFF => self.wram[(address & 0x1FFF) as usize],
            0xD000 ..= 0xDFFF | 0xF000 ..= 0xFDFF => self.wram[(address & 0x1FFF) as usize], // switchable banks later
            0xFE00 ..= 0xFE9F => self.oam[(address & 0x1FFF) as usize],
            0xFF00 ..= 0xFF7F => self.io.read_byte(address),
            0xFF80 ..= 0xFFFE => self.hram[(address & 0x1FFF) as usize],
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x7FFF => self.write_rom(address, value),
            _ => 0xFF,
        };
    }

    pub fn read_word(&mut self, address: u16) -> u16 {
        (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value * 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        match self.mbc.header.rom_type {
            MBCTypes::ROMONLY => self.mbc.rom_data[address as usize],
            MBCTypes::MBC1 => {
                match address {
                    0x4000 => 0 as u8,
                    0x6000 => 0 as u8,
                    0xA000 => 0 as u8,
                    _ => 0 as u8,
                }
            },
            // TODO: IMPLEMENT THESE
            MBCTypes::MBC2 => 0xFF,
            MBCTypes::MMM01 => 0xFF,
            MBCTypes::MBC3 => 0xFF,
            MBCTypes::MBC5 => 0xFF,
            MBCTypes::MBC6 => 0xFF,
            MBCTypes::MBC7 => 0xFF,
            MBCTypes::TAMA5 => 0xFF,
            MBCTypes::HUC1 => 0xFF,
            MBCTypes::HUC3 => 0xFF,
            MBCTypes::UNKNOWN => 0xFF,
        }
        
        
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        match self.mbc.header.rom_type {
            MBCTypes::ROMONLY => 0,
            // TODO: IMPLEMENT THESE
            MBCTypes::MBC1 => 0xFF,
            MBCTypes::MBC2 => 0xFF,
            MBCTypes::MMM01 => 0xFF,
            MBCTypes::MBC3 => 0xFF,
            MBCTypes::MBC5 => 0xFF,
            MBCTypes::MBC6 => 0xFF,
            MBCTypes::MBC7 => 0xFF,
            MBCTypes::TAMA5 => 0xFF,
            MBCTypes::HUC1 => 0xFF,
            MBCTypes::HUC3 => 0xFF,
            MBCTypes::UNKNOWN => 0xFF,
        }
    }

    pub fn write_rom(&mut self, address: u16, value: u8) -> u8 {
        match self.mbc.header.rom_type {
            MBCTypes::ROMONLY => 0,
            // TODO: IMPLEMENT THESE
            MBCTypes::MBC1 => 0xFF,
            MBCTypes::MBC2 => 0xFF,
            MBCTypes::MMM01 => 0xFF,
            MBCTypes::MBC3 => 0xFF,
            MBCTypes::MBC5 => 0xFF,
            MBCTypes::MBC6 => 0xFF,
            MBCTypes::MBC7 => 0xFF,
            MBCTypes::TAMA5 => 0xFF,
            MBCTypes::HUC1 => 0xFF,
            MBCTypes::HUC3 => 0xFF,
            MBCTypes::UNKNOWN => 0xFF,
        }
    }

    pub fn write_ram(&mut self, address: u16, value: u8) -> u8 {
        match self.mbc.header.rom_type {
            MBCTypes::ROMONLY => 0,
            // TODO: IMPLEMENT THESE
            MBCTypes::MBC1 => 0xFF,
            MBCTypes::MBC2 => 0xFF,
            MBCTypes::MMM01 => 0xFF,
            MBCTypes::MBC3 => 0xFF,
            MBCTypes::MBC5 => 0xFF,
            MBCTypes::MBC6 => 0xFF,
            MBCTypes::MBC7 => 0xFF,
            MBCTypes::TAMA5 => 0xFF,
            MBCTypes::HUC1 => 0xFF,
            MBCTypes::HUC3 => 0xFF,
            MBCTypes::UNKNOWN => 0xFF,
        }
    }
}