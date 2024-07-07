mod vectors;
use fast_image_resize::{images::Image, IntoImageView, PixelType};
use tracing::instrument;
use vectors::*;

#[derive(Debug, Clone)]
pub struct AsciifyOptions {
    resolution: Vec2<u32>,
    resizer: fast_image_resize::Resizer,
}

impl AsciifyOptions {
    pub fn new() -> Self {
        Self {
            resolution: Vec2 { x: 80, y: 40 },
            resizer: fast_image_resize::Resizer::new(),
        }
    }

    pub fn resize(&mut self, resolution: Vec2<u32>) {
        self.resolution = resolution;
    }
}

// pub struct Asciify {
//     options: AsciifyOptions,
// }
impl AsciifyOptions {
    #[instrument(level = "trace", skip(input))]
    pub fn scale<'b>(&'_ mut self, input: &'b impl IntoImageView) -> anyhow::Result<Image<'b>> {
        use fast_image_resize::images::Image;
        let mut output_image = Image::new(self.resolution.x, self.resolution.y, PixelType::U8x3);
        self.resizer.resize(input, &mut output_image, None)?;
        Ok(output_image)
    }
}

#[instrument(level = "trace", fields(input = "input.len()"))]
pub fn asciify(input: &impl IntoImageView, mut options: AsciifyOptions) -> anyhow::Result<String> {
    let scaled = options.scale(input)?;
    let luminanced = luminance(scaled.buffer());
    let quantized = quantize(&luminanced, 8);
    let ascii = SHADER_MAP8.shade_all(&quantized);
    let ascii = ascii
        .chunks_exact(scaled.width() as usize)
        .map(|line| {
            let mut line = line.to_vec();
            line.push(b'\n');
            line
        })
        .flatten();
    // let quantized_normal = u8x1_to_u8x3(&quantized);
    // let quantized = Image::from_vec_u8(
    //     scaled.width(),
    //     scaled.height(),
    //     quantized_normal,
    //     PixelType::U8x3,
    // )?;
    let ascii = String::from_utf8(ascii.collect())?;

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

/// This function quantizes the pixel values to a given number of levels
pub fn quantize(pixels: &[u8], levels: u8) -> Vec<u8> {
    pixels
        .iter()
        .map(|&pixel| {
            let step = 255 / levels;
            let level = pixel / step;
            level - 1
        })
        .collect()
}

pub fn u8x1_to_u8x3(pixels: &[u8]) -> Vec<u8> {
    pixels
        .iter()
        .flat_map(|&pixel| [pixel, pixel, pixel])
        .collect()
}

impl<const SIZE: usize> ShaderMap<SIZE> {
    #[inline(always)]
    pub fn shade(&self, value: u8) -> u8 {
        self.map[value as usize]
    }
    pub fn shade_all(&self, values: &[u8]) -> Vec<u8> {
        values.iter().map(|&value| self.shade(value)).collect()
    }
}

pub struct ShaderMap<const SIZE: usize> {
    pub map: [u8; SIZE],
}

const SHADER_MAP8: ShaderMap<8> = ShaderMap {
    map: [b' ', b'.', b':', b'-', b'=', b'*', b'#', b'@'],
};
