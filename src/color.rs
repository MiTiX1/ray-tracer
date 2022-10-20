use crate::vec3::Vec3;

pub fn write_color(color: Vec3) {
    println!(
        "{} {} {}", 
        (255.999 * color.x()) as i32,
        (255.999 * color.y()) as i32,
        (255.999 * color.z()) as i32
    );
}