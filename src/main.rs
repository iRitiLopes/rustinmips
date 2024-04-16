mod instructions;
use crate::instructions::Instruction;

#[derive(Clone)]
struct Register {
    value: u32,
}

impl Register {
    fn new() -> Register {
        Register { value: 0 }
    }

    fn read(&self) -> u32 {
        self.value
    }

    fn write(&mut self, value: u32) {
        self.value = value;
    }
}

struct Memory {
    data: Vec<u8>,
}

impl Memory {
    fn new(size: usize) -> Memory {
        Memory {
            data: vec![0; size],
        }
    }

    fn read(&self, address: u32) -> u8 {
        self.data[address as usize]
    }

    fn read_byte(&self, address: u32) -> u32 {
        self.data[address as usize] as u32
    }

    fn write(&mut self, address: u32, value: u8) {
        self.data[address as usize] = value;
    }
}

pub struct CPU {
    registers: Vec<Register>,
    memory: Memory,
    pc: u32,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            registers: vec![Register::new(); 32],
            memory: Memory::new(1024),
            pc: 0,
        }
    }
}

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Registers:\n")?;
        for (i, register) in self.registers.iter().enumerate() {
            write!(f, "Register {}: {}\n", i, register.read())?;
        }
        Ok(())
    }
}
fn main() {
    let mut cpu = CPU::new();
    let instruction = instructions::r_instructions::RTypeInstruction::new(0x1098020);
    instruction.execute(&mut cpu);
}
