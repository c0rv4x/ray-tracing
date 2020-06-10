// fn format_pixel(r: u32, g: u32, b: u32) -> u32 {
//     return r + g + b;
// }

mod vec3;
use vec3::Vec3;

fn main() {
    let xsize: u32 = 100;
    let ysize: u32 = 100;

    println!("P3");
    println!("{} {}", xsize, ysize);
    println!("255");
    for j in (0..ysize).rev() {
        for i in 0..xsize {
            let r: f32 = i as f32 / xsize as f32;
            let g: f32 = j as f32 / ysize as f32;
            let b: f32 = 0.2;
            let col = Vec3::new(r, g, b);

            let r: i32 = (256 as f32 * col.r()) as i32;
            let g: i32 = (256 as f32 * col.g()) as i32;
            let b: i32 = (256 as f32 * col.b()) as i32;
            println!("{} {} {}", r, g, b);
        }
    }
}
