use crate::vec3::Vec3;

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min }
    if x > max { return max }
    x
}

pub fn write_color(color: Vec3, samples_per_pixel: f32) {
    let mut r: f32 = color.x();
    let mut g: f32 = color.y();
    let mut b: f32 = color.z();

    let scale: f32 = 1.0 / samples_per_pixel;
    r *= scale; 
    g *= scale;
    b *= scale;

    println!(
        "{} {} {}", 
        (255.999 * clamp(r, 0.0, 0.999)) as i32,
        (255.999 * clamp(g, 0.0, 0.999)) as i32,
        (255.999 * clamp(b, 0.0, 0.999)) as i32
    );
}