use crate::ray;
use ray::Ray;


pub struct HitRecord {
    pub t: f32
}

pub trait Hitable {
    fn hits(&self, r: &Ray, tmin: f32, tmax: f32, rec: &HitRecord) -> bool;
}
