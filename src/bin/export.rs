use std::fs::File;
use std::io::Write;
use bincode::config;
use clap::Parser;
use engine::assets::Assets;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, ignore_errors = true)]
pub struct Args {
    /// The directory in which to find assets
    #[arg(short, long)]
    pub source: String,
    /// The filename for the finished assets file
    #[arg(short, long)]
    pub output: String
}

pub fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut file_output = File::create(&args.output)?;

    let mut assets = Assets::new();
    assets.load_from_filesystem(&args.source);

    let binary_data = bincode::encode_to_vec(&assets, config::standard()).unwrap();
    file_output.write(binary_data.as_slice())?;
    file_output.flush()
}