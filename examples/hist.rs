use chrono::Utc;
use csv::{ReaderBuilder, StringRecord};
use int_data_analysis::kmeans::{KMeans, Model};
use ndarray::Array2;
use plotters::prelude::*;
use std::collections::HashMap;
use std::error::Error;

const CLUSTERS: usize = 4;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path("data/results.csv")?;
    let records: Vec<StringRecord> = reader.records().map(|r| r.unwrap()).collect();
    let data = records_into_array(&records);

    println!("Clusters - Inertia");
    let mut models: HashMap<u32, Model> = HashMap::new();
    for n_clusters in 1..=6 {
        let model = KMeans::default().n_clusters(n_clusters).fit(&data);
        println!("{n_clusters} - {}", model.inertia());
        models.insert(n_clusters, model);
    }

    let best_model = models.get(&4).unwrap();
    println!("Result\n{:?}", best_model.centroids());

    let now = Utc::now().format("(%Y-%m-%d %H:%M:%S)").to_string();
    let filepath = format!("figures/hist {}.svg", now);
    create_plot("Partition problem", filepath, best_model, &data)?;

    Ok(())
}

fn create_plot(
    caption: &str,
    filepath: String,
    model: &Model,
    data: &Array2<f64>,
) -> Result<(), Box<dyn Error>> {
    //TODO: sort clusters (or use linfa)
    let root = SVGBackend::new(&filepath, (400, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart_ctx = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 20).into_font())
        .margin(5)
        .set_left_and_bottom_label_area_size(20)
        .build_cartesian_2d((0..CLUSTERS - 1).into_segmented(), 0..data.nrows() + 1)?;

    chart_ctx
        .configure_mesh()
        .light_line_style(&WHITE.mix(0.3))
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

pub fn records_into_array(records: &Vec<StringRecord>) -> Array2<f64> {
    let shape = (records.len(), records[0].len());
    let vec: Vec<f64> = records
        .iter()
        .flat_map(|rec| rec.iter().map(|str| str.parse().unwrap()))
        .collect();
    Array2::from_shape_vec(shape, vec).unwrap()
}

