use crate::CPU;

pub mod i_instructions;
pub mod j_instructions;
pub mod r_instructions;

pub trait Instruction {
    fn decode(&self) -> String;
    fn execute(&self, cpu: &mut CPU);
}
trait Executable<T> {
    fn execute(&self, r_instruction: T, cpu: &mut CPU);
}


pub fn get_instruction(word: u32) -> Box<dyn Instruction> {
    let opcode = word >> 26;
    match opcode {
        0 => Box::new(r_instructions::RTypeInstruction::new(word)),
        2 | 3 => Box::new(j_instructions::JTypeInstruction::new(word)),
        _ => Box::new(i_instructions::ITypeInstruction::new(word)),
    }
}