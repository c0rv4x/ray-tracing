mod vec3;
mod hitable;
use vec3::Vec3;
use hitable::Hitable;


pub struct Sphere {
    center: Vec3,
    radius: f32
}


impl Hitable for Sphere {
    fn hits(&self, r: &Ray, tmin: f32, tmax: f32, rec: &hit_record) {
        
    }
}