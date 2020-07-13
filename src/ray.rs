use crate::vec3;
use vec3::Vec3;

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
    pub fn hit_sphere(&self, center: Vec3, radius: f32) -> bool {
        let delta: Vec3 = self.origin - center;
        let a: f32 = self.direction() * self.direction();
        let b: f32 = 2.0 * (delta * self.direction());
        let c: f32 = delta * delta - radius * radius;
        let discriminant: f32 = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}

pub fn color(r: Ray) -> Vec3 {
    if r.hit_sphere(Vec3::new(0.0, 0.0, -4.0), 0.5) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let direction: Vec3 = r.direction().unit_vector();
    let t: f32 = 0.5 * (direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
