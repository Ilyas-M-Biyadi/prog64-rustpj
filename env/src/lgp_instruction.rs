
use rand::Rng;

use crate::{pcgrng::PCGRng};



pub const OPCODE_SIZE: u64 = 2; // 2 bits
pub const RESULT_SIZE: u64 = 1; // 1 bit
pub const OPERAND_SIZE: u64 = 3; // 3 bits
pub const INSTRUCTION_SIZE: u64 = OPCODE_SIZE + RESULT_SIZE + 2*OPERAND_SIZE ; // 9 bits

pub const OPERATIONS_NUMBER: u64 = 4; // 4 operations: add, sub, mul, div
pub const REGISTERS_NUMBER: u64 = 6; // 6 registers: R0, R1, R2, R3, R4, R5
pub const INPUT_REGISTERS_NUMBER: u64 = 4; // 4 input registers: I0, I1, I2, I3

pub const OPCODE_MASK: u64 = (1 << OPCODE_SIZE) - 1; // 2 bits
pub const RESULT_MASK: u64 = (1 << RESULT_SIZE) - 1; // 1 bit
pub const OPERAND_MASK: u64 = (1 << OPERAND_SIZE) - 1;  // 3 bits

pub const NB_INSTRUCTIONS: u64 = 64 / INSTRUCTION_SIZE; // 6 instructions per program

pub const NB_MUTATED_BITS: i8 = 8; //Nomrbe de bits à 1 dans le masque qui sera utilisé pour muter les programmes.

enum _Opcode {
    Add = 0,
    Sub = 1,
    Mul = 2,
    Div = 3,
}

#[derive(Clone)]
pub struct Instruction {
    opcode: u8,    //2bits
    result: u8,    //1bit
    operand1: u8,  //3bits
    operand2: u8,  //3bits
}

impl Instruction {
    pub fn new(opcode: u8, result: u8, operand1: u8, operand2: u8) -> Self {
        Instruction {
            opcode,
            result,
            operand1,
            operand2,
        }
    }

    pub fn new_random(rng: & mut PCGRng) -> Self {
        Instruction {
            opcode: (rng.next_u64() % OPERATIONS_NUMBER) as u8,
            result: (rng.next_u64() % (REGISTERS_NUMBER - INPUT_REGISTERS_NUMBER)) as u8,
            operand1: (rng.next_u64() % REGISTERS_NUMBER) as u8,
            operand2: (rng.next_u64() % REGISTERS_NUMBER) as u8,
        }
    }

    pub fn new_random_u64(rng: & mut PCGRng) -> u64 {
        rng.next_u64() as u64
    }

    pub fn get_opcode(&self) -> u8 {
        self.opcode
    }

    pub fn get_result(&self) -> u8 {
        self.result
    }

    pub fn get_operand1(&self) -> u8 {
        self.operand1
    }

    pub fn get_operand2(&self) -> u8 {
        self.operand2
    }

    pub fn encode(&self)-> u64 {
        (self.opcode as u64) << INSTRUCTION_SIZE - OPCODE_SIZE |
        (self.result as u64) << INSTRUCTION_SIZE - OPCODE_SIZE - RESULT_SIZE |
        (self.operand1 as u64) << INSTRUCTION_SIZE - OPCODE_SIZE - RESULT_SIZE - OPERAND_SIZE| 
        (self.operand2 as u64)
    }

    pub fn decode(encoded: u64) -> Self {
        let opcode = ((encoded >> INSTRUCTION_SIZE - OPCODE_SIZE) & OPCODE_MASK) as u8;
        let result = ((encoded >> INSTRUCTION_SIZE - OPCODE_SIZE - RESULT_SIZE) & RESULT_MASK) as u8;
        let operand1 = (((encoded >> INSTRUCTION_SIZE - OPCODE_SIZE - RESULT_SIZE - OPERAND_SIZE) & OPERAND_MASK) % REGISTERS_NUMBER)as u8;
        let operand2 = ((encoded & OPERAND_MASK) % REGISTERS_NUMBER) as u8;
        Instruction {
            opcode,
            result,
            operand1,
            operand2,
        }
    }

    //R[8] les 4 premiers les registres et les 4 derniers les inputs
    pub fn execute(&self, _r: &mut [f32; REGISTERS_NUMBER as usize]) {
        _r[self.result as usize] = match self.opcode {
            0 => _r[self.operand1 as usize] + _r[self.operand2 as usize], // Add
            1 => _r[self.operand1 as usize] - _r[self.operand2 as usize], // Sub
            2 => _r[self.operand1 as usize] * _r[self.operand2 as usize], // Mul
            3 => {
                if _r[self.operand2 as usize] != 0.0 {
                    _r[self.operand1 as usize] / _r[self.operand2 as usize] // Div
                } else {
                    0.0
                }
            }
            _ => panic!("Invalid opcode"),
        }
    }

    pub fn display(&self) {
        let op = match self.opcode {
            0 => "+",
            1 => "-",
            2 => "x",
            3 => "/",
            _ => "Invalid opcode",
        };
        println!("R[{}] = R[{}] {} R[{}]\t", self.result, self.operand1, op, self.operand2);
    }

}