use crate::core::cartridge::Cartridge;
use crate::core::mmu::Mmu;
use crate::core::opcodes::Opcode;
use crate::core::registers::Registers;


pub struct Cpu {
    pub reg: Registers,
    pub mmu: Mmu,
    pub is_halted: bool,
    pub cycles: u32,
    pub is_cgb: bool,
    pub is_paused: bool,
}


impl Cpu {
    pub fn new(cartridge: Cartridge, is_cgb: bool) -> Self {
        Self {
            reg: Registers::new(&is_cgb),
            mmu: Mmu::new(cartridge),
            is_halted: false,
            cycles: 0,
            is_cgb,
            is_paused: false,
        }
    }

    pub fn execute(&mut self) {
        while !self.is_paused {
            self.update_interrupts();
            self.decode();
            // self.mmu.ppu.execute();
            self.get_serial_message();
        }
    }

    fn get_serial_message(&mut self) {
        if self.mmu.io.is_there_serial_data() {
            let serial_data = self.mmu.io.get_serial_data();
            let serial_string = serial_data.escape_ascii().to_string();
            println!("{}", serial_string);
        }
    }

    fn decode(&mut self) {
        let instruction: u8;
        (instruction, self.reg.pc) = self.mmu.fetch_instruction(&mut self.reg.pc);
        let mut opcode = Opcode::new(instruction, &mut self.reg, &mut self.mmu);
        opcode.decode();
        if opcode.is_halted {
            self.is_halted = true;
        }
        self.cycles += opcode.get_cycles();
        
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;

    
}
