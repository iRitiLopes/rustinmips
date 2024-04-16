struct JTypeInstruction {
    opcode: u8,
    name: String,
    address: u32,
}

impl JTypeInstruction {
    fn new(instruction: u32) -> JTypeInstruction {
        JTypeInstruction {
            opcode: (instruction >> 26) as u8,
            name: String::from("J-Type"),
            address: instruction & 0x3FFFFFF,
        }
    }

    fn decode(&self) -> String {
        format!("{} {}", self.name, self.address)
    }
}
