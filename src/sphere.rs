use crate::vec3;
use vec3::Vec3;

use crate::ray;
use ray::Ray;

use crate::hitable;
use hitable::HitRecord;
use hitable::Hitable;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Hitable for Sphere {
    fn hits(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let delta: Vec3 = r.origin() - self.center;
        let a: f32 = r.direction() * r.direction();
        let b: f32 = 2.0 * (delta * r.direction());
        let c: f32 = delta * delta - self.radius * self.radius;
        let discriminant: f32 = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let tcurrent: f32 = (-b - discriminant.sqrt()) / (2.0 * a);
        if tcurrent < tmax && tcurrent > tmin {
            let p: Vec3 = r.point_at_parameter(tcurrent);
            return Some(HitRecord {
                t: tcurrent,
                p: p,
                normal: (p - self.center) / self.radius,
            });
        }

        let tcurrent: f32 = (-b + discriminant.sqrt()) / (2.0 * a);
        if tcurrent < tmax && tcurrent > tmin {
            let p: Vec3 = r.point_at_parameter(tcurrent);
            return Some(HitRecord {
                t: tcurrent,
                p: p,
                normal: (p - self.center) / self.radius,
            });
        }
        return None;
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}
