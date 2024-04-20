use crate::CPU;

use crate::instructions::Executable;
use crate::instructions::Instruction;

use super::j_instructions::JTypeInstruction;

#[derive(Clone)]
pub struct ITypeInstruction {
    opcode: u8,
    name: String,
    rs: u8,
    rt: u8,
    imm: i16,
    funct: IFunction,
}

impl ITypeInstruction {
    pub fn new(instruction: u32) -> ITypeInstruction {
        ITypeInstruction {
            opcode: (instruction >> 26) as u8,
            name: IFunction::new((instruction >> 26) as u8).name.clone(),
            rs: ((instruction >> 21) & 0b11111) as u8,
            rt: ((instruction >> 16) & 0b11111) as u8,
            imm: (instruction & 0xFFFF) as i16,
            funct: IFunction::new((instruction >> 26) as u8),
        }
    }

    fn build(opcode: u8, rs: u8, rt: u8, imm: i16) -> ITypeInstruction {
        ITypeInstruction {
            opcode,
            name: IFunction::new(opcode).name.clone(),
            rs,
            rt,
            imm,
            funct: IFunction::new(opcode),
        }
    }
}

impl Instruction for ITypeInstruction {
    fn decode(&self) -> String {
        format!("{} rs {}, rt {}, imm {}", self.name, self.rs, self.rt, self.imm)
    }

    fn execute(&self, cpu: &mut crate::CPU) {
        self.funct.execute(self.clone(), cpu);
    }
}

#[derive(Clone)]
struct IFunction {
    funct: u8,
    name: String,
}

impl IFunction {
    fn new(funct: u8) -> IFunction {
        IFunction {
            funct,
            name: match funct {
                0b001000 => String::from("ADDI"),
                0b001001 => String::from("ADDIU"),
                0b001100 => String::from("ANDI"),
                0b001101 => String::from("ORI"),
                0b000100 => String::from("BEQ"),
                0b000101 => String::from("BNE"),
                0b000001 => String::from("BGEZ"),
                0b000110 => String::from("BLEZ"),
                0b100000 => String::from("LB"),
                0b100101 => String::from("LH"),
                0b100101 => String::from("LHU"),
                0b001111 => String::from("LUI"),
                0b100011 => String::from("LW"),
                0b001010 => String::from("SLTI"),
                0b101011 => String::from("SW"),
                _ => String::from("UNKNOWN"),
            },
        }
    }

    fn decode(&self) -> String {
        format!("{} {}", self.name, self.funct)
    }
}

impl Executable<ITypeInstruction> for IFunction {
    fn execute(&self, instruction: ITypeInstruction, cpu: &mut crate::CPU) {
        match self.funct {
            // ADDI
            0b001000 => {
                let rs = cpu.registers[instruction.rs as usize].read();
                let imm = instruction.imm as u32;
                cpu.write_register(instruction.rt as usize,rs.wrapping_add(imm));
            }

            // ADDIU
            0b001001 => {
                let rs = cpu.registers[instruction.rs as usize].read();
                let imm = instruction.imm as u32;
                cpu.write_register(instruction.rt as usize,rs.wrapping_add(imm));
            }

            // ANDI
            0b001100 => {
                let rs = cpu.registers[instruction.rs as usize].read();
                let imm = instruction.imm as u32;
                cpu.write_register(instruction.rt as usize,rs & imm);
            }

            // ORI
            0b001101 => {
                let rs = cpu.registers[instruction.rs as usize].read();
                let imm = instruction.imm as u32;
                cpu.write_register(instruction.rt as usize,rs | imm);
            }

            // BEQ
            0b000100 => {
                let rs = cpu.registers[instruction.rs as usize].read();
                let rt = cpu.registers[instruction.rt as usize].read();

                if rs == rt {
                    cpu.run_branch_delayed();
                    cpu.pc = cpu.pc.wrapping_add((instruction.imm as u32) << 2);
                }
            }

            // BNE
            0b000101 => {
                let rs = cpu.registers[instruction.rs as usize].read();
                let rt = cpu.registers[instruction.rt as usize].read();

                if rs != rt {
                    cpu.run_branch_delayed();
                    cpu.pc = cpu.pc.wrapping_add((instruction.imm as u32) << 2);
                }
            }

            // BGEZ
            0b000001 => {
                let rs = cpu.registers[instruction.rs as usize].read();
                let rt = cpu.registers[instruction.rt as usize].read();
                if rs >= rt {
                    cpu.pc = cpu.pc.wrapping_add((instruction.imm as u32) << 2);
                }
            }

            // BLEZ
            0b000110 => {
                let rs = cpu.registers[instruction.rs as usize].read();
                let rt = cpu.registers[instruction.rt as usize].read();
                if rs <= rt {
                    cpu.pc = cpu.pc.wrapping_add((instruction.imm as u32) << 2);
                }
            }

            // LB
            0b100000 => {
                let rs = cpu.registers[instruction.rs as usize].read();
                let imm = instruction.imm as u32;
                let address = rs.wrapping_add(imm);
                let value = cpu.memory.read_byte(address);
                cpu.write_register(instruction.rt as usize,value as u32);
            }

            // LUI
            0b001111 => {
                let imm = instruction.imm as u32;
                cpu.write_register(instruction.rt as usize,imm << 16);
            }
            _ => panic!("Unknown IType instruction, {}", self.funct),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::Instruction;

    #[test]
    fn test_addi() {
        let mut cpu = super::CPU::new();
        let instruction = super::ITypeInstruction::build(0b001000, 2, 3, 20);

        let value: u32 = 0b1111_1111_1111_1111_1111_1111_1111_0110; // -10
        cpu.registers[instruction.rs as usize].write(value);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rt as usize].read(), 10);
    }

    #[test]
    fn test_addiu() {
        let mut cpu = super::CPU::new();
        let instruction = super::ITypeInstruction::build(0b001001, 2, 3, 1);

        let value: u32 = 0b1111_1111_1111_1111_1111_1111_1111_0110; // 4294967286
        cpu.registers[instruction.rs as usize].write(value);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rt as usize].read(), 4294967287);
    }

    #[test]
    fn test_andi() {
        let mut cpu = super::CPU::new();
        let instruction = super::ITypeInstruction::build(0b001100, 2, 3, 0b1100);

        let value: u32 = 0b0110;
        cpu.registers[instruction.rs as usize].write(value);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rt as usize].read(), 0b0100);
    }

    #[test]
    fn test_ori() {
        let mut cpu = super::CPU::new();
        let instruction = super::ITypeInstruction::build(0b001101, 2, 3, 0b1100);

        let value: u32 = 0b0110;
        cpu.registers[instruction.rs as usize].write(value);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rt as usize].read(), 0b1110);
    }

    #[test]
    fn test_beq() {
        let mut cpu = super::CPU::new();
        cpu.pc = 8;

        let instruction = super::ITypeInstruction::build(0b000100, 2, 3, 2);

        let value: u32 = 0b0110;
        cpu.registers[instruction.rs as usize].write(value);
        cpu.write_register(instruction.rt as usize,value);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.pc, 16);
    }

    #[test]
    fn test_bne() {
        let mut cpu = super::CPU::new();
        cpu.pc = 8;

        let instruction = super::ITypeInstruction::build(0b000101, 2, 3, 2);

        let value: u32 = 0b0110;
        cpu.registers[instruction.rs as usize].write(value);
        cpu.write_register(instruction.rt as usize,value + 1);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.pc, 16);
    }

    #[test]
    fn test_bgez() {
        let mut cpu = super::CPU::new();
        cpu.pc = 8;

        let instruction = super::ITypeInstruction::build(0b000001, 2, 3, 2);

        cpu.registers[instruction.rs as usize].write(3 as u32);
        cpu.write_register(instruction.rt as usize,2 as u32);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.pc, 16);
    }

    #[test]
    fn test_bgez_false() {
        let mut cpu = super::CPU::new();
        cpu.pc = 8;

        let instruction = super::ITypeInstruction::build(0b000001, 2, 3, 2);

        cpu.registers[instruction.rs as usize].write(1 as u32);
        cpu.write_register(instruction.rt as usize,2 as u32);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.pc, 8);
    }

    #[test]
    fn test_blez() {
        let mut cpu = super::CPU::new();
        cpu.pc = 8;

        let instruction = super::ITypeInstruction::build(0b000110, 2, 3, 2);

        cpu.registers[instruction.rs as usize].write(1 as u32);
        cpu.write_register(instruction.rt as usize,2 as u32);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.pc, 16);
    }

    #[test]
    fn test_blez_false() {
        let mut cpu = super::CPU::new();
        cpu.pc = 8;

        let instruction = super::ITypeInstruction::build(0b000110, 2, 3, 2);

        cpu.registers[instruction.rs as usize].write(3 as u32);
        cpu.write_register(instruction.rt as usize,2 as u32);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.pc, 8);
    }

    #[test]
    fn test_lb() {
        let mut cpu = super::CPU::new();
        let instruction = super::ITypeInstruction::build(0b100000, 2, 3, 2);

        let value: u32 = "d".as_bytes()[0] as u32;
        cpu.registers[instruction.rs as usize].write(0);
        cpu.memory.write(2, value as u32);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.registers[instruction.rt as usize].read(), value);
    }
}
