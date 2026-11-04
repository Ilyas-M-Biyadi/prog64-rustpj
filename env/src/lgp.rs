use env::{cartpole::CartPole, Environment, pcgrng::PCGRng};

fn decode_line(line: String, state: State) -> i32 {
    let mut Reg : [f32; 2] = [0, 0];
    

    let op=&line[0..2];
    let ResReg=&line[2..3].parse::<i8>().unwrap();
    let a1=[&line[3..4].parse::<i8>().unwrap(), &line[4..6].parse::<i8>().unwrap()];
    let a2=[&line[6..7].parse::<i8>().unwrap(), &line[7..9].parse::<i8>().unwrap()];

    match op {
        "00" => {
            Reg[ResReg] = Reg[a1] + Reg[a2];
        }
        "01" => {
            Reg[ResReg] = Reg[a1] - Reg[a2];
        }
        "10" => {
            Reg[ResReg] = Reg[a1] * Reg[a2];
        }
        "11" => {
            Reg[ResReg] = Reg[a1] / Reg[a2];
        }

    }
}