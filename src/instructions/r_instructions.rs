use crate::CPU;

use crate::instructions::Executable;
use crate::instructions::Instruction;


#[derive(Clone)]
pub struct RTypeInstruction {
    opcode: u8,
    name: String,
    pub rd: u8,
    pub rs: u8,
    pub rt: u8,
    shamt: u8,
    funct: RFunction,
}

impl Instruction for RTypeInstruction {
    fn decode(&self) -> String {
        format!(
            "opcode: {}, name: {} rd: {}, rs: {}, rt: {}, funct: {}",
            self.opcode,
            self.name,
            self.rd,
            self.rs,
            self.rt,
            self.funct.decode()
        )
    }

    fn execute(&self, cpu: &mut CPU) {
        self.funct.execute(self.clone(), cpu);
    }
    
}

impl RTypeInstruction {
    pub fn new(instruction: u32) -> RTypeInstruction {
        RTypeInstruction {
            opcode: (instruction >> 26) as u8,
            name: String::from("R-Type"),
            rs: ((instruction >> 21) & 0b11111) as u8,
            rt: ((instruction >> 16) & 0b11111) as u8,
            rd: ((instruction >> 11) & 0b11111) as u8,
            shamt: ((instruction >> 6) & 0b11111) as u8,
            funct: RFunction::new((instruction & 0b111111) as u8),
        }
    }

    pub fn build(opcode: u8, rd: u8, rs: u8, rt: u8, shamt: u8, funct: u8) -> RTypeInstruction {
        RTypeInstruction {
            opcode: opcode,
            name: String::from("R-Type"),
            rd: rd,
            rs: rs,
            rt: rt,
            shamt: shamt,
            funct: RFunction::new(funct),
        }
    }
}

#[derive(Clone)]
struct RFunction {
    funct: u8,
    name: String,
}

impl RFunction {
    fn new(funct: u8) -> RFunction {
        RFunction {
            funct: funct,
            name: match funct {
                0x20 => String::from("add"),
                0x21 => String::from("addu"),
                0x22 => String::from("sub"),
                0x24 => String::from("and"),
                0x25 => String::from("or"),
                0x26 => String::from("xor"),
                0x27 => String::from("nor"),
                0x2A => String::from("slt"),
                0x00 => String::from("sll"),
                0x02 => String::from("srl"),
                0x03 => String::from("sra"),
                0x08 => String::from("jr"),
                _ => String::from("unknown"),
            },
        }
    }

    fn decode(&self) -> String {
        format!("funct: {}, name: {}", self.funct, self.name)
    }
}



impl Executable<RTypeInstruction> for RFunction {
    fn execute(&self, r_instruction: RTypeInstruction, cpu: &mut CPU) {
        match self.funct {
            // Add
            0x20 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write(rs.wrapping_add(rt));
            }

            // Add Unsigned
            0x21 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write(rs.wrapping_add(rt));
            
            },

            // Subtract
            0x22 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write(rs.wrapping_sub(rt));
            
            },

            // And
            0x24 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write(rs & rt);
            
            },

            // Or
            0x25 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write(rs | rt);
            
            
            },

            // Xor
            0x26 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write(rs ^ rt);
            },

            // Nor
            0x27 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write(!(rs | rt));
            
            },

            // Set Less Than
            0x2A => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write(if rs < rt { 1 } else { 0 });
            },

            // Shift Left Logical
            0x00 => {
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write(rt << r_instruction.shamt);
            },

            // Shift Right Logical
            0x02 => {
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write(rt >> r_instruction.shamt);
            },

            // Shift Right Arithmetic
            0x03 => {
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.registers[r_instruction.rd as usize].write((rt as i32 >> r_instruction.shamt as i32) as u32);
            },

            // Jump Register
            0x08 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                cpu.pc = rs;
            },
            _ => println!("unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::Instruction;

    #[test]
    fn test_add() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 0, 0x20);

        let value: u32 = 0b1111_1111_1111_1111_1111_1111_1111_0110; // -10
        cpu.registers[instruction.rs as usize].write(value);
        cpu.registers[instruction.rt as usize].write(20);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 10);
    }

    #[test]
    fn test_sub() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 0, 0x22);
        cpu.registers[instruction.rs as usize].write(20);
        cpu.registers[instruction.rt as usize].write(10);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 10);
    }

    #[test]
    fn test_and() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 0, 0x24);
        cpu.registers[instruction.rs as usize].write(0b1010);
        cpu.registers[instruction.rt as usize].write(0b1100);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 0b1000);
    }

    #[test]
    fn test_or() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 0, 0x25);
        cpu.registers[instruction.rs as usize].write(0b1010);
        cpu.registers[instruction.rt as usize].write(0b1100);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 0b1110);
    }

    #[test]
    fn test_xor() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 0, 0x26);
        cpu.registers[instruction.rs as usize].write(0b1010);
        cpu.registers[instruction.rt as usize].write(0b1100);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 0b0110);
    }

    #[test]
    fn test_nor() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 0, 0x27);
        cpu.registers[instruction.rs as usize].write(0b00000);
        cpu.registers[instruction.rt as usize].write(0b00001);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 0b11111111111111111111111111111110);
    }

    #[test]
    fn test_slt() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 0, 0x2A);
        cpu.registers[instruction.rs as usize].write(10);
        cpu.registers[instruction.rt as usize].write(20);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 1);
    }

    #[test]
    fn test_slt_false() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 0, 0x2A);
        cpu.registers[instruction.rs as usize].write(20);
        cpu.registers[instruction.rt as usize].write(10);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 0);
    }

    #[test]
    fn test_sll() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 2, 0x00);
        cpu.registers[instruction.rt as usize].write(0b1111);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 0b111100);
    }

    #[test]
    fn test_srl() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 2, 0x02);
        cpu.registers[instruction.rt as usize].write(0b1111);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 0b11);
    }

    #[test]
    fn test_sra() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 1, 2, 3, 2, 0x03);
        let value: u32 = 0b1111_1111_1111_1111_1111_1111_1111_0110; // -10
        cpu.registers[instruction.rt as usize].write(value as u32);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rd as usize].read(), 0b1111_1111_1111_1111_1111_1111_1111_1101);
    }

    #[test]
    fn test_jr() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 0, 1, 2, 0, 0x08);
        cpu.registers[instruction.rs as usize].write(0x100);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.pc, 0x100);
    }
}
