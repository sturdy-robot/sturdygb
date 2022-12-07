use crate::core::opcodes::Opcode;
use crate::core::registers::FFlags;


impl<'a> Opcode<'a> {
    fn get_flags(&mut self) -> String {
        let mut flags = "".to_string();
        let c = self.write_flag_values(self.reg.get_flag(FFlags::C), FFlags::C);
        let h = self.write_flag_values(self.reg.get_flag(FFlags::H), FFlags::H);
        let n = self.write_flag_values(self.reg.get_flag(FFlags::N), FFlags::N);
        let z = self.write_flag_values(self.reg.get_flag(FFlags::Z), FFlags::Z);
        
        flags.push(c);
        flags.push(h);
        flags.push(n);
        flags.push(z);
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
            '_'
        }
    }

    pub fn debug_registers(&mut self) {
        let pc = self.reg.pc - 1;
        print!("[0x{:04X}]: 0x{:02X}", pc, self.opcode);
        print!("  A: 0x{:02X} F: 0x{:02X}, B: 0x{:02X} C: 0x{:02X} D: 0x{:02X} E: 0x{:02X} H: 0x{:02X} L: 0x{:02X}", self.reg.a, self.reg.f, self.reg.b, self.reg.c, self.reg.d, self.reg.e, self.reg.h, self.reg.l);
        print!("  AF: 0x{:04X} BC: 0x{:04X} DE: 0x{:04X} HL: 0x{:04X}", self.reg.af(), self.reg.bc(), self.reg.de(), self.reg.hl());
        print!("  Flags: {}", self.get_flags());
        print!("\n");
    }
}