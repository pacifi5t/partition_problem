use crate::binary_vec::BinaryVec;
use crate::target_fn;
use rand::prelude::ThreadRng;
use rand::{thread_rng, Rng};

const ALPHA: f64 = 0.95;

pub fn demon_alg(values: &Vec<f64>) -> BinaryVec {
    let (mut rng, mut demon_energy, mut x) = set_up(values);
    let (mut accepted, mut rejected, mut quasi_by_rejected) = (0, 0, 0);

    loop {
        let flip = x.one_flip();
        let y = &flip[rng.gen_range(0..flip.len())];
        let delta_f = target_fn(y, values) - target_fn(&x, values);

        if delta_f < demon_energy {
            x = y.clone();
            demon_energy -= delta_f;
            accepted += 1;
        } else {
            rejected += 1;
        }

        if let Some(was_rejected) = quasi_equilibrium(accepted, rejected, flip.len()) {
            demon_energy *= ALPHA;
            if was_rejected {
                quasi_by_rejected += 1;
            }
        }

        if quasi_by_rejected == 3 {
            break;
        }
    }

    x
}

fn set_up(values: &Vec<f64>) -> (ThreadRng, f64, BinaryVec) {
    let x = BinaryVec::random(values.len());
    (thread_rng(), init_demon_energy(&x), x)
}

fn init_demon_energy(bv: &BinaryVec) -> f64 {
    todo!()
}

fn quasi_equilibrium(accepted: usize, rejected: usize, flip_len: usize) -> Option<bool> {
    let was_rejected = rejected == 2 * flip_len;
    match accepted == flip_len || was_rejected {
        true => Some(was_rejected),
        false => None,
    }
}
