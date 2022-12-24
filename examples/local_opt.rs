use chrono::Utc;
use clap::Parser;
use int_data_analysis::kmeans::{KMeans, Model};
use ndarray::Array2;
use partition_problem::*;
use plotters::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir, File};
use std::io::Read;

#[derive(Parser, Debug)]
struct Args {
    /// Path on input file
    #[arg(value_hint = clap::ValueHint::FilePath)]
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let values = parse_file(&args.file)?;
    let results = benchmark(&values, 20);

    print_results(&values, &results);
    cluster_results(&results, 3)
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

fn benchmark(values: &Vec<f64>, runs: usize) -> Vec<f64> {
    let mut results = Vec::new();

    for i in 1..=runs {
        let res = hill_climb_min(values);
        let target_fn_val = target_fn(&res, values);
        results.push(target_fn_val);

        println!("Run {}: {:.4}", i, target_fn_val);
    }

    results
}

fn print_results(values: &Vec<f64>, results: &Vec<f64>) {
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

    let max_fn = target_fn(&BinaryVec::ones(values.len()), values);
    let base = max_fn - worst.1;
    println!("Diff: {:.4}%", (((max_fn - best.1) - base) / base) * 100.0);
}

fn cluster_results(results: &Vec<f64>, clusters: usize) -> Result<(), Box<dyn Error>> {
    let data = Array2::from_shape_vec([results.len(), 1], results.clone())?;

    println!("\nClusters - Inertia");
    let mut models: HashMap<u32, Model> = HashMap::new();
    for n_clusters in 1..=5 {
        let model = KMeans::default().n_clusters(n_clusters).fit(&data);
        println!("{n_clusters} - {:.4}", model.inertia());
        models.insert(n_clusters, model);
    }

    let best_model = models.get(&(clusters as u32)).unwrap();
    println!("\nBest model\n{:?}", best_model);

    let now = Utc::now().format("(%H:%M:%S %d.%m.%Y)").to_string();
    let filepath = format!("figures/local_opt {}.svg", now);
    create_dir("figures").unwrap_or(());
    build_histogram("Partition problem", filepath, best_model, &data)
}

fn build_histogram(
    caption: &str,
    filepath: String,
    model: &Model,
    data: &Array2<f64>,
) -> Result<(), Box<dyn Error>> {
    let root = SVGBackend::new(&filepath, (400, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart_ctx = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 20).into_font())
        .margin(5)
        .set_left_and_bottom_label_area_size(20)
        .build_cartesian_2d(
            (0..model.centroids().nrows() - 1).into_segmented(),
            0..data.nrows() + 1,
        )?;

    chart_ctx
        .configure_mesh()
        .light_line_style(WHITE.mix(0.3))
        .x_label_formatter(&|s| match s {
            SegmentValue::CenterOf(n) => format!("{:.4}", model.centroids().get([*n, 0]).unwrap()),
            _ => String::new(),
        })
        .draw()?;

    chart_ctx.draw_series(
        Histogram::vertical(&chart_ctx)
            .style(BLUE.filled())
            .margin(10)
            .data(data.rows().into_iter().map(|r| (model.predict(r), 1))),
    )?;

    root.present()?;
    Ok(())
}
