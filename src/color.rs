use crate::vectors::Vec3;
use crate::intervals::*;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Transltate the [0, 1] component values to the byte range [0,255]
    let intensity: Interval = Interval::new(0.000, 0.999);
    let rbyte: i32 = (256.0 * intensity.clamp(r)) as i32;
    let gbyte: i32 = (256.0 * intensity.clamp(g)) as i32;
    let bbyte: i32 = (256.0 * intensity.clamp(b)) as i32;

    // Write out the pixel color components.
    println!("{rbyte} {gbyte} {bbyte}");
}

