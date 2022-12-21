use super::io::IO;
use super::mbc::Mbc;
use super::ppu::Ppu;

pub struct Mmu {
    pub mbc: Box<dyn Mbc>,
    pub ppu: Ppu,
    pub io: IO,
    pub ieflag: u8,
    wram: [u8; 0x8000],
    hram: [u8; 0x7F],
    ram_bank: usize,
}

impl Mmu {
    pub fn new(cartridge: Box<dyn Mbc>) -> Self {
        Self {
            mbc: cartridge,
            ppu: Ppu::new(),
            io: IO::new(),
            ieflag: 0,
            wram: [0; 0x8000],
            hram: [0; 0x7F],
            ram_bank: 1,
        }
    }

    pub fn fetch_instruction(&mut self, pc: &mut u16) -> (u8, u16) {
        let instruction = self.read_byte(*pc);
        let inc_pc = pc.wrapping_add(1);
        (instruction, inc_pc)
    }

    pub fn push_stack(&mut self, sp: u16, address: u16) -> u16 {
        let new_sp = sp.wrapping_sub(2).clone();
        self.write_word(new_sp, address);
        new_sp
    }

    pub fn pop_stack(&mut self, sp: &mut u16) -> (u16, u16) {
        let mut temp_sp = sp.clone();
        let res = self.read_word(temp_sp);
        temp_sp = temp_sp.wrapping_add(2);
        (temp_sp, res)
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.mbc.read_rom(address),
            0x8000..=0x9FFF => self.ppu.read_byte(address),
            0xA000..=0xBFFF => self.mbc.read_ram(address),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[(address & 0x1FFF) as usize],
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.ram_bank * 0x1000) | address as usize & 0x0FFF], // switchable banks later
            0xFE00..=0xFE9F => self.ppu.read_byte(address),
            0xFF00..=0xFF26 => self.io.read_byte(address),
            0xFF40..=0xFF4F => self.ppu.read_byte(address),
            0xFF51..=0xFF55 => self.ppu.read_byte(address),
            0xFF68..=0xFF69 => self.ppu.read_byte(address),
            0xFF70 => self.ram_bank as u8,
            0xFF80..=0xFFFE => self.hram[(address & 0x007F) as usize],
            0xFFFF => self.ieflag,
            _ => 0x00,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.write_rom(address, value),
            0x8000..=0x9FFF => self.ppu.write_byte(address, value),
            0xA000..=0xBFFF => self.mbc.write_ram(address, value),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[(address & 0x1FFF) as usize] = value,
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.ram_bank * 0x1000) | address as usize & 0x0FFF] = value, // switchable banks later
            0xFE00..=0xFE9F => self.ppu.write_byte(address, value),
            0xFF00..=0xFF26 => self.io.write_byte(address, value),
            0xFF40..=0xFF4F => self.ppu.write_byte(address, value),
            0xFF51..=0xFF55 => self.ppu.write_byte(address, value),
            0xFF68..=0xFF69 => self.ppu.write_byte(address, value),
            0xFF70 => { self.ram_bank = match value & 0x7 { 0 => 1, n => n as usize }; }
            0xFF80..=0xFFFE => self.hram[(address & 0x007F) as usize] = value,
            0xFFFF => self.ieflag = value,
            _ => println!("Attempted to write to invalid memory address: 0x{address:04X}"),
        };
    }

    pub fn read_word(&mut self, address: u16) -> u16 {
        (self.read_byte(address) as u16) | ((self.read_byte(address + 1) as u16) << 8)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }
}
