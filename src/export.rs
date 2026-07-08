use std::fs::File;
use std::io::Write;
use clap::Parser;
use crate::assets::Assets;

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

pub fn export_assets() -> std::io::Result<()> {
    let args = Args::parse();
    let mut file_output = File::create(&args.output)?;

    let mut assets = Assets::new();
    assets.load_from_filesystem(&args.source);

    let binary_data = serde_json::to_vec(&assets)?;
    file_output.write(binary_data.as_slice())?;
    file_output.flush()
}