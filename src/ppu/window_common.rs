pub struct Color(pub f32, pub f32, pub f32);

pub fn rgb(color: &Color) -> u32 {
    let r = clamp((color.0 * 256.0) as u32, 0, 255);
    let g = clamp((color.1 * 256.0) as u32, 0, 255);
    let b = clamp((color.2 * 256.0) as u32, 0, 255);
    r << 16 | g << 8 | b << 0
}

pub fn pack(r: u8, g: u8, b: u8) -> u32 {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    (r << 16 | g << 8 | b << 0)
}

fn clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
    if v < min { min } else if v > max { max } else { v }
}