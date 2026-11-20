use std::vec;

use env::{cartpole::CartPole, Environment, pcgrng::PCGRng, lgp_instruction::Instruction, lgp_program::Program};

fn main() {
    let nb_programs: usize = 10;
    let seed = 12344;
    let mut rng = PCGRng::seed(seed);
    let mut rng_programs = PCGRng::seed(1);
    let mut env = CartPole::default();
    let mut total_reward:Vec<f32> = Vec::with_capacity(nb_programs);
    let mut reg : [f32; 6] = [0.0; 6];  //registers R0: result, R1, R2, R3  // R4, R5 input registers I0, I1, I2, I3
    

    let mut programs: Vec<Program> = Vec::with_capacity(nb_programs);

    let mut encoded_progs: Vec<u64> = Vec::with_capacity(nb_programs);

/*     for i in 0..nb_programs {
        programs.push(Program::new_random(&mut rng_programs));
        encoded_progs.push(programs[i].encode());
    }


    for i in 0..nb_programs {
        loop {
            programs[i].execute(&mut reg);
            let action = (reg[0]) as f32;
            let (_, reward, done) = env.step(action, &mut rng);

            total_reward[i] += reward;

            if done {
                break;
            }
        }
        println!("Programme {}: Total reward: {}", i, total_reward[i]);
    }
*/
    //println!("Total reward: {}", total_reward)

 /*    let inst: Instruction = Instruction::new(0, 1, 7, 7);

    let encoded = inst.encode();

    println!("Encoded instruction: {}\n", encoded);

    let decoded = Instruction::decode(encoded);

    decoded.display();

    inst.execute(&mut reg);

    for i in 0..6 {
        println!("R[{}] = {}", i, reg[i]);
    }

    println!("Result = R[{}] = {}", inst.get_result(), reg[inst.get_result() as usize]);
*/

    let random = Instruction::new_random_u64(&mut rng);

    let random_decoded = Instruction::decode(random);

    random_decoded.display();

    reg[0] = 0.0;
    reg[1] = 1.0;
    reg[2] = 2.0;
    reg[3] = 3.0;
    reg[4] = 4.0;
    reg[5] = 5.0;
    //Execute random instruction
    let random_inst = Instruction::new_random(&mut rng);
    //random_inst.display();
    random_inst.execute(&mut reg);
    println!("Result = R[{}] = {}", random_inst.get_result(), reg[random_inst.get_result() as usize]);

    //let prog: Program = Program::new_random(&mut rng);

    //prog.display();
    
}
