use binary_vec::BinaryVec;
use std::ops::Sub;

mod binary_vec;

fn target_fn(bvec: &BinaryVec, values: &Vec<f64>) -> f64 {
    let iter = bvec.data.iter().enumerate();
    let sum1 = iter
        .map(|(i, elem)| match elem {
            true => values.get(i).unwrap().clone(),
            false => 0.0,
        })
        .sum::<f64>();

    let sum2 = values.iter().sum::<f64>() / 2.0;

    sum1.sub(sum2).abs()
}

fn hill_climb_min(values: &Vec<f64>) -> BinaryVec {
    let mut x = BinaryVec::random(values.len());
    let mut found = true;

    while found {
        let mut min = f64::MAX;
        let mut y = x.clone();

        for each in x.one_flip() {
            let temp = target_fn(&each, &values);
            if temp < min {
                min = temp;
                y = each;
            }
        }

        if target_fn(&y, &values) < target_fn(&x, &values) {
            x = y;
        } else {
            found = false;
        }
    }

    x
}

fn main() {
    let costs = vec![1.2, 4.1, 3.4, 2.0];
    let res = hill_climb_min(&costs);
    println!("{:?}", res.data);
    println!("{}", target_fn(&res, &costs))
}
