use std::fs::{create_dir, File};
use std::io::Write;
use rand::Rng;

fn main() -> std::io::Result<()> {
    let filepath = "data/jewels.txt";

    std::fs::remove_file(filepath).unwrap_or(());
    create_dir("data").unwrap_or(());

    let mut rng = rand::thread_rng();
    let mut buf = String::new();
    for _ in 0..200 {
        buf += format!("{}\n", rng.gen_range(10.0..1_000.0)).as_str();
    }

    let mut file = File::create(filepath)?;
    file.write_all(buf.as_bytes())?;
    Ok(())
}
