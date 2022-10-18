use crate::core::mmu::MMU;
use crate::core::registers::{CPUFlags, Registers, ByteRegister, WordRegister};
use crate::core::cartridge::Cartridge;
use crate::core::opcodes::Opcode;

pub struct CPU {
    pub reg: Registers,
    pub mmu: MMU,
    pub is_halted: bool,
    pub cycles: u8,
    pub is_cgb: bool,
    pub ime: bool,
}

impl CPU {
    pub fn new(cartridge: Cartridge, is_cgb: bool) -> Self {
        Self {
            reg: Registers::new(&is_cgb),
            mmu: MMU::new(cartridge),
            is_halted: false,
            cycles: 0,
            is_cgb,
            ime: true,
        }
    }

    pub fn execute(&mut self) {
        while !self.is_halted && self.reg.pc < 0x8000 {
            let instruction: u8;
            (instruction, self.reg.pc) = self.mmu.fetch_instruction(&mut self.reg.pc);
            let mut opcode = Opcode::new(instruction, &mut self.reg, &mut self.mmu);
            opcode.decode();
            if opcode.is_halted {
                self.is_halted = true;
                break;
            }
            self.reg.pc = self.reg.pc.wrapping_add(1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::CPU;
    use crate::core::cartridge::{Cartridge, CartridgeHeader, MBCTypes};
    use crate::core::mmu::MMU;
    use crate::core::registers::Registers;

    fn set_up(rom_data: Vec<u8>) -> CPU {
        let cartridge_header = CartridgeHeader {
            entry: [0; 4],
            logo: [0; 0x30],
            title: [0; 16],
            cgb_flag: 0x80,
            licensee_code: "".to_string(),
            sgb_flag: 0x00,
            rom_type: MBCTypes::ROMONLY,
            rom_size: 0x00,
            ram_size: 0x00,
            dest_code: 0x00,
            checksum: 0x00,
        };
        let cartridge = Cartridge {
            header: cartridge_header,
            rom_data: rom_data,
            ram: Vec::new(),
        };

        CPU {
            cycles: 0,
            ime: true,
            is_cgb: false,
            is_halted: false,
            mmu: MMU::new(cartridge),
            reg: Registers::new(&false),
        }
    }
}
