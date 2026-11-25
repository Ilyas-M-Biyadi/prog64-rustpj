use rand::Rng;

use crate::{pcgrng::PCGRng, lgp_instruction::NB_MUTATED_BITS};


    pub fn gen_mask(rng: &mut PCGRng) -> u64 {
        let mut mask: u64 = 0;
        let _k=NB_MUTATED_BITS;
        for _i in 0..(_k+1) {
            let rand_ind=(rng.next_u32() % 64) as i32;
            mask |= 1u64 << rand_ind;
        }
        mask
    }