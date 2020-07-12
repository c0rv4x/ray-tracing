mod ray;
mod vec3;
use ray::color;
use ray::Ray;
use vec3::Vec3;

fn main() {
    let xsize: u32 = 200;
    let ysize: u32 = 100;

    println!("P3");
    println!("{} {}", xsize, ysize);
    println!("255");

    let left_corner: Vec3 = Vec3::new(-2.0, -1.0, -1.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0, 0.0);
    for j in (0..ysize).rev() {
        for i in 0..xsize {
            let u: f32 = i as f32 / xsize as f32;
            let v: f32 = j as f32 / ysize as f32;
            let direction_ray: Ray = Ray::new(origin, left_corner + u * horizontal + v * vertical);

            let color_multipliers: Vec3 = color(direction_ray);
            let red: usize = (255.99 * color_multipliers.r()) as usize;
            let green: usize = (255.99 * color_multipliers.g()) as usize;
            let blue: usize = (255.99 * color_multipliers.b()) as usize;
            println!("{} {} {}", red, green, blue);
            // color_multipliers.print();
        }
    }
}
