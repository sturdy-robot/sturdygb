use crate::core::opcodes::Opcode;
use crate::core::registers::FFlags;


impl<'a> Opcode<'a> {
    fn get_flags(&mut self) -> String {
        let mut flags = "".to_string();
        let c = self.write_flag_values(self.reg.get_flag(FFlags::C), FFlags::C);
        let h = self.write_flag_values(self.reg.get_flag(FFlags::H), FFlags::H);
        let n = self.write_flag_values(self.reg.get_flag(FFlags::N), FFlags::N);
        let z = self.write_flag_values(self.reg.get_flag(FFlags::Z), FFlags::Z);
        
        flags.push(z);
        flags.push(n);
        flags.push(h);
        flags.push(c);
        
        
        flags
    }

    fn write_flag_values(&mut self, flag_value: u8, flag: FFlags) -> char {
        if flag_value == 1 {
            match flag {
                FFlags::C => 'C',
                FFlags::H => 'H',
                FFlags::N => 'N',
                FFlags::Z => 'Z',
            }
        } else {
            '-'
        }
    }

    pub fn debug_registers(&mut self) {
        let pc = self.reg.pc;
        print!("[{:04X}]: ({:02X} {:02X} {:02X})", pc, self.opcode, self.mmu.read_byte(pc.wrapping_add(1)), self.mmu.read_byte(pc.wrapping_add(2)));
        // print!("  A: 0x{:02X} F: 0x{:02X}, B: 0x{:02X} C: 0x{:02X} D: 0x{:02X} E: 0x{:02X} H: 0x{:02X} L: 0x{:02X}", self.reg.a, self.reg.f, self.reg.b, self.reg.c, self.reg.d, self.reg.e, self.reg.h, self.reg.l);
        print!(" A: {:02X}", self.reg.a);
        print!(" F: {}", self.get_flags());
        print!(" BC: {:04X} DE: {:04X} HL: {:04X}", self.reg.bc(), self.reg.de(), self.reg.hl());
        print!(" SP: {:04X}", self.reg.sp);
        print!("\n");
    }
}