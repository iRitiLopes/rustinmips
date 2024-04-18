use crate::instructions::Executable;
use crate::instructions::Instruction;
use crate::CPU;

#[derive(Clone)]
pub struct JTypeInstruction {
    opcode: u8,
    name: String,
    address: u32,
    funct: JFunction
}

impl JTypeInstruction {
    pub fn new(instruction: u32) -> JTypeInstruction {
        JTypeInstruction {
            opcode: (instruction >> 26) as u8,
            name: JFunction::new((instruction >> 26) as u8).name.clone(),
            address: instruction & 0x3FFFFFF,
            funct: JFunction::new((instruction >> 26) as u8)
        }
    }

    fn build(opcode: u8, address: u32) -> JTypeInstruction {
        JTypeInstruction {
            opcode,
            name: JFunction::new(opcode).name.clone(),
            address,
            funct: JFunction::new(opcode)
        }
    }
}

impl Instruction for JTypeInstruction {
    fn decode(&self) -> String {
        format!("{} {} {}", self.name, self.funct.decode(), self.address)
    }

    fn execute(&self, cpu: &mut crate::CPU) {
        self.funct.execute(self.clone(), cpu);
    }
}


#[derive(Clone)]
struct JFunction {
    opcode: u8,
    name: String
}


impl JFunction {
    fn new(opcode: u8) -> JFunction {
        JFunction {
            opcode,
            name: String::from("J-Type")
        }
    }

    fn decode(&self) -> String {
        format!("{} {}", self.name, self.opcode)
    }
}

impl Executable<JTypeInstruction> for JFunction {
    fn execute(&self, instruction: JTypeInstruction, cpu: &mut crate::CPU) {
        match self.opcode {
            0b000010 => {
                cpu.pc = instruction.address << 2;
            }

            0b000011 => {
                let new_address = instruction.address << 2;
                let ra = cpu.pc + 8;
                cpu.registers[31].write(ra);
                cpu.pc = new_address;
            }
            _ => panic!("Invalid J-Type instruction")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::Instruction;

    #[test]
    fn test_j_type_instruction() {
        let mut cpu = super::CPU::new();
        let instruction = super::JTypeInstruction::new(0x08000001);
        assert_eq!(instruction.opcode, 0b000010);
        assert_eq!(instruction.address, 0x000001);

        instruction.execute(&mut cpu);

        assert_eq!(cpu.pc, 0x000001 << 2);
    }

    #[test]
    fn test_jal(){
        let mut cpu = super::CPU::new();
        let instruction = super::JTypeInstruction::new(0x0C000001);
        assert_eq!(instruction.opcode, 0b000011);
        assert_eq!(instruction.address, 0x000001);

        instruction.execute(&mut cpu);

        assert_eq!(cpu.pc, 0x000001 << 2);
        assert_eq!(cpu.registers[31].read(), 0x00000008);
    }
}