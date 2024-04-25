use std::io;
use std::io::Write;

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
    fn decode(&self, cpu: &mut CPU) -> String {
        let rd_value = cpu.read_register(self.rd as usize);
        let rs_value = cpu.read_register(self.rs as usize);
        let rt_value = cpu.read_register(self.rt as usize);
        format!("{} rd {}: {}, rs {}: {}, rt {}: {}", self.name, self.rd, rd_value, self.rs, rs_value, self.rt, rt_value)
    }

    fn execute(&self, cpu: &mut CPU) {
        self.funct.execute(self.clone(), cpu);
    }
}

impl RTypeInstruction {
    pub fn new(instruction: u32) -> RTypeInstruction {
        RTypeInstruction {
            opcode: (instruction >> 26) as u8,
            name: RFunction::new((instruction & 0b111111) as u8).name.clone(),
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
            name: RFunction::new(funct).name.clone(),
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
                0x20 => String::from("ADD"),
                0x21 => String::from("ADDU"),
                0x22 => String::from("SUB"),
                0x24 => String::from("AND"),
                0x25 => String::from("OR"),
                0x26 => String::from("XOR"),
                0x27 => String::from("NOR"),
                0x2A => String::from("SLT"),
                0x00 => String::from("SLL"),
                0x0d => String::from("NOOP"),
                0x02 => String::from("SRL"),
                0x03 => String::from("SRA"),
                0x08 => String::from("JR"),
                0x0c => String::from("SYSCALL"),
                _ => String::from(format!("unknown {} ||||", funct)),
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
                cpu.write_register(r_instruction.rd as usize,rs.wrapping_add(rt));
            }

            // Add Unsigned
            0x21 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.write_register(r_instruction.rd as usize,rs.wrapping_add(rt));
            }

            // Subtract
            0x22 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.write_register(r_instruction.rd as usize,rs.wrapping_sub(rt));
            }

            // And
            0x24 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.write_register(r_instruction.rd as usize,rs & rt);
            }

            // Or
            0x25 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.write_register(r_instruction.rd as usize,rs | rt);
            }

            // Xor
            0x26 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.write_register(r_instruction.rd as usize,rs ^ rt);
            }

            // Nor
            0x27 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.write_register(r_instruction.rd as usize,!(rs | rt));
            }

            // Set Less Than
            0x2A => {
                let rs = cpu.registers[r_instruction.rs as usize].read() as i32;
                let rt = cpu.registers[r_instruction.rt as usize].read() as i32;
                let result = rs < rt;

                //println!("SLT: {} < {}", rs, rt);
                cpu.write_register(r_instruction.rd as usize,result as u32);
            }

            // Shift Left Logical
            0x00 => {
                if r_instruction.rd == 0 && r_instruction.rt == 0 {
                    return;
                }

                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.write_register(r_instruction.rd as usize,rt << r_instruction.shamt);
            }

            0x0d => {
                // No operation
            }

            // Shift Right Logical
            0x02 => {
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.write_register(r_instruction.rd as usize,rt >> r_instruction.shamt);
            }

            // Shift Right Arithmetic
            0x03 => {
                let rt = cpu.registers[r_instruction.rt as usize].read();
                cpu.write_register(r_instruction.rd as usize, (rt as i32 >> r_instruction.shamt as i32) as u32);
            }

            // Jump Register
            0x08 => {
                cpu.run_branch_delayed();
                cpu.pc = cpu.registers[r_instruction.rs as usize].read();
                cpu.jump = true;
            }

            // Jump and Link Register
            0x09 => {
                let rs = cpu.registers[r_instruction.rs as usize].read();
                let ra = cpu.pc + 8;
                cpu.run_branch_delayed();
                cpu.write_register(r_instruction.rd as usize, ra);
                cpu.pc = rs;
                cpu.jump = true;
            }

            // Syscall
            0x0c => {
                let v0 = cpu.registers[2].read();
                let a0 = cpu.registers[4].read();

                if v0 == 1 {
                    print!("{}", a0 as u32);
                }

                if v0 == 4 {
                    let text = utils::get_text(cpu, a0);
                    print!("{:}", text);
                }

                if v0 == 5 {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input: u32 = input.trim().parse().unwrap();
                    cpu.registers[2].write(input);
                }

                if v0 == 10 {
                    std::process::exit(0);
                }

                if v0 == 11 {
                    let theChar = a0 as u8 as char;
                    print!("{:}", theChar);
                }

                io::stdout().flush().unwrap();
            }
            _ => println!("unknown"),
        }
    }
}

mod utils {
    use crate::CPU;

    pub fn get_text(cpu: &CPU, address: u32) -> String {
        let mut relative = address % 4;
        let mut address = address;
        let mut text = String::new();
        let mut must_loop = true;
        loop {
            if relative != 0 {
                address -= relative;
            }

            let value = cpu.memory.read(address);

            let mut byte_chain = Vec::<u8>::new();

            for i in (relative as usize)..4 {
                let byte = ((value >> (i * 8)) & 0xFF) as u8;
                if byte == 0 {
                    must_loop = false;
                    break
                }
                byte_chain.push(byte);
            }

            relative = 0;

            let char_chain = latin1_to_string(&byte_chain);

            text.push_str(&char_chain);

            if !must_loop {
                break;
            }

            address += 4;
        }

        text
    }

    fn latin1_to_string(s: &[u8]) -> String {
        s.iter().map(|&c| c as char).collect()
    }
}



#[cfg(test)]
mod tests {
    use crate::instructions::r_instructions::utils;
    use crate::instructions::Executable;
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
        assert_eq!(
            cpu.registers[instruction.rd as usize].read(),
            0b11111111111111111111111111111110
        );
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
        assert_eq!(
            cpu.registers[instruction.rd as usize].read(),
            0b1111_1111_1111_1111_1111_1111_1111_1101
        );
    }

    #[test]
    fn test_jr() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::build(0, 0, 1, 2, 0, 0x08);
        cpu.registers[instruction.rs as usize].write(0x100);
        instruction.execute(&mut cpu);
        assert_eq!(cpu.pc, 0x100);
    }

    #[test]
    fn test_syscall_v0_is_1() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::new(0x0c);
        let v0 = 2;
        let a0 = 4;
        cpu.registers[v0].write(1);
        cpu.registers[a0].write(10);
        instruction.execute(&mut cpu);
    }

    #[test]
    fn test_syscall_v0_is_4() {
        let mut cpu = super::CPU::new();
        let instruction = super::RTypeInstruction::new(0x0c);
        let v0 = 2;
        let a0 = 4;

        let data_address = 0x00400000;
        cpu.registers[v0].write(4);
        cpu.registers[a0].write(data_address);

        let text = "Hello\0\0\0\0".as_bytes();

        let mut word: u32 = 0;
        let mut store_address = 0x00400000;
        for (i, &byte) in text.iter().enumerate() {
            word = word | (byte as u32) << (i % 4) * 8;
            if (i + 1) % 4 == 0 {
                cpu.memory.write(store_address, word);
                store_address += 4;
                word = 0;
            }
        }
        instruction.execute(&mut cpu);

        assert_eq!(utils::get_text(&cpu, data_address), "Hello");
    }
}
