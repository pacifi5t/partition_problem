use hill_climb::*;
use std::error::Error;
use std::fs::File;
use std::io::Read;

mod binary_vec;
mod hill_climb;

fn parse_file() -> Result<Vec<f64>, Box<dyn Error>> {
    let mut file = File::open("data/jewels.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut costs = Vec::<f64>::new();
    for each in buffer.split('\n').filter(|s| !s.is_empty()) {
        costs.push(each.parse()?);
    }

    Ok(costs)
}

fn benchmark(costs: &Vec<f64>, runs: usize) {
    let mut results =  Vec::new();

    for i in 1..=runs {
        let res = hill_climb_min(costs);
        let target_fn_val = target_fn(&res, costs);
        results.push(target_fn_val);

        println!("Run {}: {}", i, target_fn_val);
    }

    let mut min = f64::MAX;
    for each in results {
        if each < min {
            min = each;
        }
    }

    println!("\nBest: {}", min);
}

fn main() -> Result<(), Box<dyn Error>> {
    let costs = parse_file()?;

    benchmark(&costs, 10);

    Ok(())
}
