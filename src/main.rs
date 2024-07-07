use ascii_shade::asciify;
mod translators;

use anyhow::Result;
use clap::*;
use std::path::PathBuf;
#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub input: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let input = zune_image::image::Image::open(&cli.input)?;
    let input = translators::zune_to_fast_image_resize(input)?;
    let options = ascii_shade::AsciifyOptions::new();
    let output = asciify(&input, options)?;
    println!("{}", output);
    Ok(())
}
