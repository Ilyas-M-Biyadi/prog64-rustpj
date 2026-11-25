//use std::vec;
use std::cmp::Reverse;

use env::{/*State, */cartpole::CartPole, Environment, pcgrng::PCGRng, lgp_instruction::REGISTERS_NUMBER, lgp_program::Program, mutation};

pub const EPOCHS: i32 = 50;

fn main() {
    const NB_PROGRAMS: usize = 100;
    let seed = 0;
    let progseed = 0;
    let mut rng = PCGRng::seed(seed);
    let mut rng_programs = PCGRng::seed(progseed);
    //let mut total_reward:[i32; NB_PROGRAMS as usize] = [0; NB_PROGRAMS as usize];
    let mut reg : [[f32; REGISTERS_NUMBER as usize]; NB_PROGRAMS as usize] = [[0.0; REGISTERS_NUMBER as usize]; NB_PROGRAMS as usize];  //registers R0: result, R1, R2, R3  // R4, R5 input registers I0, I1, I2, I3
    

    let mut programs: Vec<Program> = Vec::with_capacity(NB_PROGRAMS);

    //let mut encoded_progs: Vec<u64> = Vec::with_capacity(nb_programs);

    for i in 0..NB_PROGRAMS as usize {
        for j in 2..REGISTERS_NUMBER as usize {
            reg[i][j] = j as f32;
        }
    }

    for _i in 0..NB_PROGRAMS {
        programs.push(Program::new_u64(&mut rng_programs));
    }

    for epochs in 0..EPOCHS {

        //let mut state: State = State {x: 0.0, x_dot: 0.0, theta: 0.0, theta_dot: 0.0};
        for iter in 0..NB_PROGRAMS {
            programs[iter].decode();
            let mut env = CartPole::default();
            let mut total_reward : i32 = 0;

            loop {

                programs[iter].execute(&mut reg[iter]);
                
                let action = reg[iter][0];
                let (state, reward, done) = env.step(action, &mut rng);

                reg[iter][2] = state[0];
                reg[iter][3] = state[1];
                reg[iter][4] = state[2];
                reg[iter][5] = state[3];

                total_reward += reward as i32;

                if done {
                    programs[iter].set_reward(total_reward);
                    env.reset(&mut rng);
                    break;
                }
                    
            }
            //println!("Programme {}: Total reward: {} Program: {}", iter, programs[iter].get_reward(),programs[iter].get_64b());

        }

        println!("Epoch {}", epochs);
        programs.sort_by_key(|p| Reverse(p.get_reward()));
        println!("Best reward: {}", programs[0].get_reward());
        for _i in 10..NB_PROGRAMS {
        //programs[_i] = Program::new_u64(&mut rng_programs);
            let rand_ind=(rng.next_u32() % 10) as usize;
            programs[_i]=programs[rand_ind].clone();
            programs[_i].mutate(&mut rng_programs);
            //println!("{:064b}", programs[_i].get_64b());
        }

        for _i in 0..NB_PROGRAMS {
            programs[_i].set_reward(0);
        }
        
    }
 
        

   /* programs.sort_by_key(|p| Reverse(p.get_reward()));
    for _i in 10..NB_PROGRAMS {
        //programs[_i] = Program::new_u64(&mut rng_programs);
        let rand_ind=(rng.next_u32() % 10) as usize;
        programs[_i]=programs[rand_ind].clone();
        programs[_i].mutate(&mut rng_programs);
        println!("{:064b}", programs[_i].get_64b());
    }*/
    

    /*
    println!("Total reward: {}", programs[0].get_reward());

    for _i in 10..NB_PROGRAMS {
        programs[_i] = Program::new_u64(&mut rng_programs);
    }
    println!("{:064b}", mutation::gen_mask(&mut rng));
    */
    
    
    
    
    /*
    let mut p = Program::new_u64(&mut rng_programs);
    p.decode();
    let o=p.get_64b();
    let mu=p.mutate(&mut rng_programs).get_64b();
    let msk=o^mu;
    println!("Programme o:\t{:064b}\nProgramme m:\t{:064b}\nProgramme msk:\t{:064b}", o, mu, msk );
    */

    
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

    //let random = Instruction::new_random_u64(&mut rng);

    //let random_decoded = Instruction::decode(random);

    //random_decoded.display();


    //Execute random instruction
    //let random_inst = Instruction::new_random(&mut rng);
    //random_inst.display();
    //random_inst.execute(&mut reg);
    //println!("Result = R[{}] = {}", random_inst.get_result(), reg[random_inst.get_result() as usize]);

    //let prog: Program = Program::new(&mut rng);

    //prog.display();

    
}
