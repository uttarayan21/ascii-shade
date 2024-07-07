mod vectors;
use std::borrow::Cow;

use ascii_shade_macros::shader_map;
use fast_image_resize::{images::Image, IntoImageView, PixelType};
use tracing::instrument;
use vectors::*;

#[derive(Debug, Clone, derive_builder::Builder)]
#[builder(setter(into))]
pub struct AsciifyOptions {
    resolution: Vec2<u32>,
    #[builder(default = "fast_image_resize::Resizer::new()", setter(skip))]
    resizer: fast_image_resize::Resizer,
    #[builder(default = "None", setter(name = "resizer"))]
    resizer_options: Option<fast_image_resize::ResizeOptions>,
    shader: ShaderMap<'static>,
}

impl AsciifyOptions {
    pub fn new() -> Self {
        Self {
            resolution: Vec2 { x: 80, y: 40 },
            resizer: fast_image_resize::Resizer::new(),
            shader: SHADER_MAP_DEFAULT,
            resizer_options: None,
        }
    }

    pub fn builder() -> AsciifyOptionsBuilder {
        AsciifyOptionsBuilder::default()
    }

    pub fn resize(&mut self, resolution: Vec2<u32>) {
        self.resolution = resolution;
    }
}

impl AsciifyOptions {
    #[instrument(level = "trace", skip(input))]
    pub fn scale<'b>(&'_ mut self, input: &'b impl IntoImageView) -> anyhow::Result<Image<'b>> {
        use fast_image_resize::images::Image;
        let mut output_image = Image::new(self.resolution.x, self.resolution.y, PixelType::U8x3);
        self.resizer
            .resize(input, &mut output_image, &self.resizer_options)?;
        Ok(output_image)
    }
}

pub struct TerminalImage {
    data: Vec<char>, // Since it might be unicde, we use char
    width: usize,
}

impl core::fmt::Display for TerminalImage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for line in self.data.chunks(self.width) {
            for &c in line {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[instrument(level = "trace", fields(input = "input.len()"))]
pub fn asciify(
    input: &impl IntoImageView,
    mut options: AsciifyOptions,
) -> anyhow::Result<TerminalImage> {
    let scaled = options.scale(input)?;
    let luminanced = luminance(scaled.buffer());
    let ascii = options.shader.render(&luminanced, scaled.width() as _);
    Ok(ascii)
}

/// This function calculates how bright the pixel is
pub fn luminance(pixels: &[u8]) -> Vec<u8> {
    pixels
        .chunks_exact(3)
        .map(|pixel| {
            let r = pixel[0] as f32;
            let g = pixel[1] as f32;
            let b = pixel[2] as f32;
            // https://cs.stackexchange.com/questions/11876/how-do-i-compute-the-luminance-of-a-pixel
            let l = 0.3 * r + 0.59 * g + 0.11 * b;
            l as u8
        })
        .collect()
}

impl ShaderMap<'_> {
    #[inline(always)]
    pub fn quantize(&self, value: u8) -> u8 {
        let step = 255 / self.map.len() as u8 - 1;
        let level = value / step;
        level.saturating_sub(1)
    }

    pub fn quantize_all(&self, values: &[u8]) -> Vec<u8> {
        values.iter().map(|&value| self.quantize(value)).collect()
    }

    #[inline(always)]
    pub fn shade(&self, value: u8) -> char {
        self.map[value as usize]
    }
    pub fn shade_all(&self, values: &[u8]) -> Vec<char> {
        values.iter().map(|&value| self.shade(value)).collect()
    }

    pub fn render(&self, values: &[u8], width: usize) -> TerminalImage {
        let data = self.quantize_all(values);
        let data = self.shade_all(&data);
        TerminalImage { data, width }
    }

    pub fn from_str(map: &str) -> Self {
        Self {
            map: map.chars().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShaderMap<'s> {
    pub map: Cow<'s, [char]>,
}

pub const SHADER_MAP_DEFAULT: ShaderMap = shader_map!(" .:-=*#@");
pub const SHADER_MAP_UNICODE: ShaderMap = shader_map!(" ░▒▓█");
pub const SHADER_MAP_MIXED: ShaderMap = shader_map!(" .-=*#@░▒▓█");

// /// This function quantizes the pixel values to a given number of levels
// pub fn quantize(pixels: &[u8], levels: u8) -> Vec<u8> {
//     pixels
//         .iter()
//         .map(|&pixel| {
//             let step = 255 / levels;
//             let level = pixel / step;
//             level.saturating_sub(1)
//         })
//         .collect()
// }

// pub fn u8x1_to_u8x3(pixels: &[u8]) -> Vec<u8> {
//     pixels
//         .iter()
//         .flat_map(|&pixel| [pixel, pixel, pixel])
//         .collect()
// }
