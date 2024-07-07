use ascii_shade::asciify;
mod translators;

use anyhow::Result;
use clap::*;
use std::path::PathBuf;
#[derive(Debug, Clone, Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub input: PathBuf,
    #[clap(short, long, default_value = "80")]
    pub width: u32,
    #[clap(short, long, default_value = "40")]
    pub height: u32,
    #[clap(short, long, default_value = "default")]
    pub shader: Shader,
    #[clap(short, long, default_value = "lanczos3")]
    pub resizer: ResizeOptions,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Shader {
    Default,
    Unicode,
    Mixed,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum ResizeOptions {
    Nearest,
    Box,
    Bilinear,
    Hamming,
    CatmullRom,
    Mitchell,
    Gaussian,
    Lanczos3,
}

impl ResizeOptions {
    pub fn resize_algo(self) -> fast_image_resize::ResizeAlg {
        use fast_image_resize::{FilterType::*, ResizeAlg};

        match self {
            ResizeOptions::Nearest => ResizeAlg::Nearest,
            ResizeOptions::Box => ResizeAlg::Convolution(Box),
            ResizeOptions::Bilinear => ResizeAlg::Convolution(Bilinear),
            ResizeOptions::Hamming => ResizeAlg::Convolution(Hamming),
            ResizeOptions::CatmullRom => ResizeAlg::Convolution(CatmullRom),
            ResizeOptions::Mitchell => ResizeAlg::Convolution(Mitchell),
            ResizeOptions::Gaussian => ResizeAlg::Convolution(Gaussian),
            ResizeOptions::Lanczos3 => ResizeAlg::Convolution(Lanczos3),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let input = zune_image::image::Image::open(&cli.input)?;
    let input = translators::zune_to_fast_image_resize(input)?;
    let options = ascii_shade::AsciifyOptions::builder()
        .resolution((cli.width, cli.height))
        .shader(match cli.shader {
            Shader::Default => ascii_shade::SHADER_MAP_DEFAULT,
            Shader::Unicode => ascii_shade::SHADER_MAP_UNICODE,
            Shader::Mixed => ascii_shade::SHADER_MAP_MIXED,
        })
        .resizer(fast_image_resize::ResizeOptions::new().resize_alg(cli.resizer.resize_algo()))
        .build()?;
    let output = asciify(&input, options)?;
    use std::io::Write;
    writeln!(std::io::stdout(), "{}", output)?;
    Ok(())
}
