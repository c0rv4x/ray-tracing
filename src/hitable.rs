use crate::ray;
use ray::Ray;

use crate::vec3::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hits(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}
