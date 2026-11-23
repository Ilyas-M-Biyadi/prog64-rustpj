use rand::Rng;

use crate::{pcgrng::PCGRng};


    pub fn gen_mask(rng: &mut PCGRng, _k: i8) -> u64 {
        let mut mask: u64 = 0;
        for _i in 0..(_k+1) {
            let rand_ind=(rng.next_u32() % 64) as i32;
            mask |= 1u64 << rand_ind;
        }
        mask
    }