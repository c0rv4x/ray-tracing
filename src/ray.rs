use crate::vec3;
use vec3::Vec3;

use crate::hitable;
use hitable::HitRecord;
use hitable::Hitable;

use crate::sphere;
use sphere::Sphere;

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {
            origin: a,
            direction: b,
        }
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub fn get_background(r: &Ray) -> Vec3 {
    let direction: Vec3 = r.direction().unit_vector();
    let t: f32 = 0.5 * (direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

pub fn color(r: Ray) -> Vec3 {
    let sphere_center: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let sphere_one: Sphere = Sphere::new(sphere_center, 0.2);

    match sphere_one.hits(&r, 0.0, 50.0) {
        None => get_background(&r),
        Some(h) => Vec3::new(1.0, 0.0, 0.0)
    }

    // if hits {
    //     return Vec3::new(1.0, 0.0, 0.0);
    // }
    // if hit_value > 0.0 {
    //     let normal: Vec3 = r.point_at_parameter(hit_value) - sphere_center;

    //     return 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0));
    // }

}
