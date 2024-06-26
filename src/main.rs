mod instructions;
use crate::instructions::Instruction;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::Cursor;

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
    data: Vec<u32>,
    stack_pointer: u32,
    global_pointer: u32,
}

impl Memory {
    const STACK_POINTER: u32 = 0x7fffeffc;
    const GLOBAL_POINTER: u32 = 0x10008000;

    fn new(size: usize) -> Memory {
        Memory {
            data: vec![0; 2u64.pow(32) as usize],
            stack_pointer: Self::STACK_POINTER,
            global_pointer: Self::GLOBAL_POINTER,
        }
    }

    fn read(&self, address: u32) -> u32 {
        self.data[address as usize]
    }

    fn read_byte(&self, address: u32) -> u32 {
        self.data[address as usize] as u32
    }

    fn write(&mut self, address: u32, value: u32) {
        self.data[address as usize] = value;
    }

    fn load_text(&mut self, text: Vec<u32>) {
        let mut initial_text_address = 0x00400000;
        for (_, word) in text.iter().enumerate() {
            self.write(initial_text_address, *word);
            initial_text_address += 4;
        }
    }

    fn load_data(&mut self, data: Vec<u32>) {
        let mut initial_data_address = 0x10010000;
        for (_, word) in data.iter().enumerate() {
            self.write(initial_data_address, *word);
            initial_data_address += 4;
        }
    }
}

pub struct CPU {
    registers: Vec<Register>,
    memory: Memory,
    pc: u32,
    jump: bool,
}

impl CPU {
    fn new() -> CPU {
        let mut cpu = CPU {
            registers: vec![Register::new(); 32],
            memory: Memory::new(1024),
            pc: 0,
            jump: false,
        };

        cpu.write_register(28, cpu.memory.global_pointer);
        cpu.write_register(29, cpu.memory.stack_pointer);
        cpu
    }

    fn read_register(&self, register: usize) -> u32 {
        if register == 0 {
            return 0;
        }

        self.registers[register].read()
    }

    fn write_register(&mut self, register: usize, value: u32) {
        if register == 0 {
            return;
        }

        self.registers[register].write(value);
    }

    fn run(&mut self) {
        self.pc = 0x00400000;
        let mut count = 0;
        loop {
            let instruction = self.memory.read(self.pc);

            if instruction == 0 {
                self.pc += 4;
                continue;
            }

            let instruction = instructions::get_instruction(instruction);

            instruction.execute(self);

            if !self.jump {
                self.pc += 4;
            } else {
                self.jump = false;
            }

            count += 1;

            if count > 8000 {
                std::process::exit(0)
            }
        }
    }

    fn run_branch_delayed(&mut self) {
        let branch_delayed_instruction = self.memory.read(self.pc + 4);

        let branch_delayed_instruction = instructions::get_instruction(branch_delayed_instruction);

        branch_delayed_instruction.execute(self);
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

fn read_program_elf(cpu: &mut CPU, file_path: &str) {
    use std::fs::File;
    use std::io::Read;

    let mut data = File::open(format!("{}.data", file_path)).expect("File not found");
    let mut data_code = Vec::<u32>::new();
    while let Ok(word) = data.read_u32::<LittleEndian>() {
        data_code.push(word);
    }

    let mut text = File::open(format!("{}.text", file_path)).expect("File not found");
    let mut text_code = Vec::<u32>::new();
    while let Ok(word) = text.read_u32::<LittleEndian>() {
        text_code.push(word);
    }

    cpu.memory.load_text(text_code);
    cpu.memory.load_data(data_code);
}

fn main() {
    let mut cpu = CPU::new();

    read_program_elf(&mut cpu, "./examples/08.sort");

    cpu.run();
}
