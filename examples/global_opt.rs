use partition_problem::*;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let costs = parse_file("data/subset_sum_example.csv")?;
    let results = benchmark(&costs, 10);

    print_results(&costs, &results);
    Ok(())
}

fn parse_file(filepath: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    let mut file = File::open(filepath)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut costs = Vec::<f64>::new();
    for each in buffer.split('\n').filter(|s| !s.is_empty()) {
        costs.push(each.parse()?);
    }

    Ok(costs)
}

fn benchmark(costs: &Vec<f64>, runs: usize) -> Vec<f64> {
    let mut results = Vec::new();

    for i in 1..=runs {
        let res = demon_alg(costs);
        let target_fn_val = target_fn(&res, costs);
        results.push(target_fn_val);

        println!("Run {}: {:.4}", i, target_fn_val);
    }

    results
}

fn print_results(costs: &Vec<f64>, results: &Vec<f64>) {
    let mut best = (0, &f64::MAX);
    let mut worst = (0, &f64::MIN);
    for each in results.iter().enumerate() {
        if each.1 < best.1 {
            best = each
        }
        if each.1 > worst.1 {
            worst = each
        }
    }

    println!("\nBest run: {} - {:.4}", best.0 + 1, best.1);
    println!("Worst run: {} - {:.4}", worst.0 + 1, worst.1);

    let max_fn = target_fn(&BinaryVec::ones(costs.len()), costs);
    let base = max_fn - worst.1;
    println!("Diff: {:.4}%", (((max_fn - best.1) - base) / base) * 100.0);
}
