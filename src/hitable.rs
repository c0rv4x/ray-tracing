mod ray;
use ray::Ray;


pub struct hit_record {
    t: f32
}

pub trait hitable {
    fn hits(&self, r: &Ray, tmin: f32, tmax: f32, rec: &hit_record);
}
