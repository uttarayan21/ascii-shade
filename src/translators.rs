use anyhow::Result;
use zune_jpeg::zune_core::colorspace::ColorSpace;

pub fn zune_to_fast_image_resize(
    image: zune_image::image::Image,
) -> Result<fast_image_resize::images::Image<'static>> {
    let colorspace = image.colorspace();
    let pixel_type = match colorspace {
        ColorSpace::RGB => fast_image_resize::PixelType::U8x3,
        ColorSpace::RGBA => fast_image_resize::PixelType::U8x4,
        _ => return Err(anyhow::anyhow!("Unsupported colorspace: {colorspace:?}",)),
    };
    let (width, height) = image.dimensions();
    let mut image = image.flatten_to_u8();
    let image = image.pop().expect("Failed to get the image frame");
    let image =
        fast_image_resize::images::Image::from_vec_u8(width as _, height as _, image, pixel_type)?;
    Ok(image)
}

pub fn fast_image_resize_to_zune(
    image: fast_image_resize::images::Image,
) -> Result<zune_image::image::Image> {
    let colorspace = match image.pixel_type() {
        // fast_image_resize::PixelType::U8 => 1,
        // fast_image_resize::PixelType::U8x2 => 2,
        fast_image_resize::PixelType::U8x3 => ColorSpace::RGB,
        fast_image_resize::PixelType::U8x4 => ColorSpace::RGBA,
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported number of components: {:?}",
                image.pixel_type()
            ))
        }
    };
    let image = zune_image::image::Image::from_u8(
        image.buffer(),
        image.width() as _,
        image.height() as _,
        colorspace,
    );
    Ok(image)
}
