use rand::Rng;

use crate::{lgp_instruction::{INSTRUCTION_SIZE, Instruction, NB_INSTRUCTIONS, REGISTERS_NUMBER}, pcgrng::PCGRng};

pub struct Program {
    program_64b: u64,
    instructions: Vec<Instruction>,
    total_reward: i32,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Program { 
            program_64b: 0 as u64,
            instructions,
            total_reward: 0 as i32,

        }
    }

    pub fn new_random(rng: & mut PCGRng) -> Self {
        let mut instructions = Vec::with_capacity(NB_INSTRUCTIONS as usize);
        for _ in 0..NB_INSTRUCTIONS {
            instructions.push(Instruction::new_random(rng));
        }
        Program { 
            program_64b: 0 as u64,
            instructions,
            total_reward: 0 as i32,

        }
    }

    pub fn new_u64(rng : &mut PCGRng) -> Self {
        Program {
            program_64b: rng.next_u64() as u64,
            instructions: Vec::with_capacity(NB_INSTRUCTIONS as usize),
            total_reward: 0 as i32,

        }
    }

    pub fn get_64b(&self) -> u64{
        self.program_64b
    }
    pub fn get_reward(&self) -> i32{
        self.total_reward
    }

     pub fn set_reward(&mut self, v: i32) {
        self.total_reward=v;
    }

    pub fn encode(&mut self) -> &mut Self {

        for (i, instruction) in self.instructions.iter().enumerate() {
            self.program_64b |= instruction.encode() << (64 - (i*INSTRUCTION_SIZE as usize));
        }

        self
    }

    pub fn decode(&mut self) -> &mut Self {

        for i in 0..NB_INSTRUCTIONS {
            let instruction_encoded = (self.program_64b >> (64 - ((i+1)*INSTRUCTION_SIZE))) & ((1 << INSTRUCTION_SIZE) - 1);
            self.instructions.push(Instruction::decode(instruction_encoded));
        }

        self
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

