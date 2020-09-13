mod ray;
use ray::Ray;

pub trait Hitable {
    fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool;
}
