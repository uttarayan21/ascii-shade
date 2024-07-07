# Ascii Shade

This is a preliminary implementaion of a ascii image converter

### How it works
The image is first scaled down to an appropirate level for a terminal (80x40)
then we first calculate the luminance of each pixel
then we quantize each pixel then we map the values to an ascii shader map

The pseudocode is as follows
```rust
const SHADER: [u8; 8] = *b" .:-=*#@";
let pixels: Vec<u8> = image::read("myimage.jpg"); // Lets say this is a u8x3 pixel array
let scaled: Vec<u8> = image::resize(pixels, og_w, og_h, new_w, new_h); // This is still a u8x3 pixel array
let luminance: Vec<u8> = scaled.chunks_exact(3).map(|pixel| pixel[0] *.3 + pixel[1] * .59 + pixel[2] * .11).collect();
/// We just get the quantization level instead of actually quantizing since that'll be easier to index the shader array
let quantized = luminance.iter().map(|l| (l / (255 / 8)) - 1).collect();
let final = quantized.iter().map(|q| SHADER[q]).collect::<Vec<_>>().chunks_exact(new_w).map(|line| line.push('\n')).collect();
```

### Why ?
- **Yes**
