use crate::binary_vec::BinaryVec;
use crate::target_fn;
use rand::{thread_rng, Rng};

const ALPHA: f64 = 0.95;

pub fn demon_alg(values: &Vec<f64>) -> BinaryVec {
    let mut rng = thread_rng();
    let mut x = BinaryVec::random(values.len());
    let mut demon_energy = init_demon_energy(&x);

    loop {
        let flip = x.one_flip();
        let y = flip[rng.gen_range(0..flip.len())];
        let delta_f = target_fn(&y, values) - target_fn(&x, values);

        if delta_f < demon_energy {
            x = y;
            demon_energy -= delta_f;
        }

        if is_quasi_equilibrium() {
            demon_energy *= ALPHA;
        }

        if is_frozen() {
            break;
        }
    }

    x
}

fn init_demon_energy(bv: &BinaryVec) -> f64 {
    todo!()
}

fn is_quasi_equilibrium() -> bool {
    todo!()
}

fn is_frozen() -> bool {
    todo!()
}
