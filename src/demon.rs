use crate::binary_vec::BinaryVec;
use crate::target_fn;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

const ALPHA: f64 = 0.95;
const SEED: u64 = 42;

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
            accepted = 0;
            rejected = 0;
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

fn set_up(values: &Vec<f64>) -> (Xoshiro256Plus, f64, BinaryVec) {
    let rng = Xoshiro256Plus::seed_from_u64(SEED);
    let x = BinaryVec::random(values.len());
    (rng, init_demon_energy(values, &x), x)
}

fn init_demon_energy(values: &Vec<f64>, bv: &BinaryVec) -> f64 {
    let precision = 0.01;
    let mut rng = Xoshiro256Plus::seed_from_u64(SEED);
    let mut energy = f32::MAX as f64;
    let mut delta = energy / 2.0;

    loop {
        let res = demon_try(&mut rng, values, bv, energy);
        if res > 0.9 + precision {
            energy -= delta;
        } else if res < 0.9 - precision {
            energy += delta;
        } else {
            return energy;
        }

        delta /= 2.0;
    }
}

fn demon_try(rng: &mut Xoshiro256Plus, values: &Vec<f64>, bv: &BinaryVec, energy: f64) -> f64 {
    const ATTEMPTS: usize = 100;

    let flip = bv.one_flip();
    let mut accepted = 0;

    for _ in 0..ATTEMPTS {
        let y = &flip[rng.gen_range(0..flip.len())];
        let delta_f = target_fn(y, values) - target_fn(bv, values);

        if delta_f < energy {
            accepted += 1;
        }
    }

    accepted as f64 / ATTEMPTS as f64
}

fn quasi_equilibrium(accepted: usize, rejected: usize, flip_len: usize) -> Option<bool> {
    let was_rejected = rejected == 2 * flip_len;
    match accepted == flip_len || was_rejected {
        true => Some(was_rejected),
        false => None,
    }
}
