use binary_vec::BinaryVec;

pub mod binary_vec;
pub mod demon;
pub mod hill_climb;

pub fn target_fn(bin_vec: &BinaryVec, values: &Vec<f64>) -> f64 {
    let nums = bin_vec.as_nums();
    let sum1 = nums
        .iter()
        .enumerate()
        .map(|(i, e)| *values.get(i).unwrap() * *e as f64)
        .sum::<f64>();

    let sum2 = values.iter().sum::<f64>() / 2.0;

    (sum1 - sum2).abs()
}
