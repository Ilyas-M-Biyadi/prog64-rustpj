use crate::{lgp_instruction::{INSTRUCTION_SIZE, Instruction, NB_INSTRUCTIONS, REGISTERS_NUMBER}, pcgrng::PCGRng};

pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Program { instructions }
    }

    pub fn new_random(rng: & mut PCGRng) -> Self {
        let mut instructions = Vec::with_capacity(NB_INSTRUCTIONS as usize);
        for _ in 0..NB_INSTRUCTIONS {
            instructions.push(Instruction::new_random(rng));
        }
        Program { instructions }
    }

    pub fn encode(&self) -> u64 {
        let mut _program_64b: u64 = 0;

        for (i, instruction) in self.instructions.iter().enumerate() {
            _program_64b |= instruction.encode() << (64 - (i*INSTRUCTION_SIZE as usize));
        }

        return _program_64b as u64;
    }

    pub fn decode(encoded_program: u64) -> Self {
        let mut instructions = Vec::with_capacity(NB_INSTRUCTIONS as usize);
        for i in 0..NB_INSTRUCTIONS {
            let instruction_encoded = (encoded_program >> (64 - ((i+1)*INSTRUCTION_SIZE))) & ((1 << INSTRUCTION_SIZE) - 1);
            instructions.push(Instruction::decode(instruction_encoded));
        }
        Program { instructions }
    }

    pub fn execute(&self, _r: &mut [f32; REGISTERS_NUMBER as usize]) {
        for instruction in &self.instructions {
            instruction.execute(_r);
        }
    }

    pub fn display(&self) {
        for (i, instruction) in self.instructions.iter().enumerate() {
            println!("Instruction {}: ", i);
            instruction.display();
        }
    }
}