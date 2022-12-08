use crate::core::cartridge::Cartridge;
use crate::core::mmu::Mmu;
use crate::core::opcodes::Opcode;
use crate::core::registers::Registers;
use crate::core::interrupts::Interrupt;


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
            self.get_serial_data();
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

    fn get_serial_data(&mut self) {
        if self.mmu.io.is_there_serial_data() {
            let serial_data = self.mmu.io.get_serial_data();
            let serial_string = String::from_utf8(serial_data).expect("No Serial data available!");
            println!("{serial_string}");
        }
    }

    fn check_interrupt(&mut self) -> bool {
        if self.reg.ime {
            let ifflag = self.mmu.io.ifflag;
            let ieflag = self.mmu.ieflag;
            ifflag & ieflag != 0
        } else {
            false
        }
    }

    fn update_interrupts(&mut self) {
        if self.check_interrupt() {
            if self.is_halted {
                self.is_halted = false;
            }
            self.reg.sp = self.mmu.push_stack(self.reg.sp, self.reg.pc);
            let interrupt = self.get_interrupt();
            self.handle_interrupt(interrupt);
        }
    }

    fn get_interrupt(&mut self) -> Interrupt {
        match self.mmu.ieflag & 0x1F {
            0x10 => Interrupt::Hitolo,
            0x08 | 0x18 => Interrupt::Serial,
            0x04 | 0x0C  => Interrupt::Timer,
            0x02 | 0x06 | 0x0E | 0x1E => Interrupt::Lcdc,
            0x01 | 0x03 | 0x07 | 0x0F | 0x1F => Interrupt::Vblank,
            _ => unreachable!(),
        }
    }

    fn handle_interrupt(&mut self, interrupt: Interrupt) {
        self.cycles += 8;
        match interrupt {
            Interrupt::Vblank => {
                self.reg.pc = 0x40;
            }
            Interrupt::Serial => {
                self.reg.pc = 0x58;
            }
            Interrupt::Hitolo => {
                self.reg.pc = 0x60;
            }
            Interrupt::Lcdc => {
                self.reg.pc = 0x48;
            }
            Interrupt::Timer => {
                self.reg.pc = 0x50;
            }
        };

        self.reg.ime = false;
    }
}

#[cfg(test)]
mod test {
    use super::Cpu;

    
}
