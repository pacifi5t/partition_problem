use super::target_fn;
use crate::binary_vec::BinaryVec;

pub fn hill_climb_min(values: &Vec<f64>) -> BinaryVec {
    let mut x = BinaryVec::random(values.len());
    let mut found = true;

    while found {
        let mut min = f64::MAX;
        let mut y = x.clone();

        for each in x.one_flip() {
            let temp = target_fn(&each, values);
            if temp < min {
                min = temp;
                y = each;
            }
        }

        if target_fn(&y, values) < target_fn(&x, values) {
            x = y;
        } else {
            found = false;
        }
    }

    x
}
