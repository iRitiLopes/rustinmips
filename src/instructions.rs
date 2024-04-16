use crate::CPU;

pub mod i_instructions;
pub mod r_instructions;
pub mod j_instructions;

pub trait Instruction {
    fn decode(&self) -> String;
    fn execute(&self, cpu: &mut CPU);
}
trait Executable<T> {
    fn execute(&self, r_instruction: T, cpu: &mut CPU);
}