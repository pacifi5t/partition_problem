use chrono::Utc;
use partition_problem::*;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let costs = parse_file("data/subsetSumExample2.csv")?;
    let (best, target_fn_data) = demon_alg(&costs);
    let target_fn_val = target_fn(&best, &costs);

    println!("{:.4}", target_fn_val);
    let now = Utc::now().format("(%y-%m-%d %H:%M:%S)").to_string();
    let filepath = format!("figures/global_opt {}.svg", now);
    plot_func("Partition problem", filepath, &target_fn_data)?;
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

fn plot_func(caption: &str, filepath: String, data: &Vec<f64>) -> Result<(), Box<dyn Error>> {
    let root = SVGBackend::new(&filepath, (1440, 900)).into_drawing_area();
    root.fill(&WHITE)?;

    let max = data.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
    let min = data.iter().min_by(|a, b| a.total_cmp(b)).unwrap();

    let mut chart_ctx = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 20).into_font())
        .margin(5)
        .set_left_and_bottom_label_area_size(50)
        .build_cartesian_2d(0..data.len(), *min..*max)?;

    chart_ctx.configure_mesh().draw()?;
    chart_ctx.draw_series(LineSeries::new(
        data.iter().enumerate().map(|e| (e.0, *e.1)),
        BLUE,
    ))?;

    root.present()?;
    Ok(())
}
