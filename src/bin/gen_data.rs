use clap::Parser;
use rand::Rng;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
struct Args {
    /// Path to output file
    #[arg(value_hint = clap::ValueHint::FilePath)]
    file: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut rng = rand::thread_rng();
    let mut buf = String::new();
    for _ in 0..200 {
        buf += format!("{}\n", rng.gen_range(0.0..1.0)).as_str();
    }

    let mut file = File::create(args.file)?;
    file.write_all(buf.as_bytes())?;
    Ok(())
}
