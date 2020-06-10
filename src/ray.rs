mod vec3;
use vec3::Vec3;

pub struct Ray {
    A: Vec3,
    B: Vec3,
}

impl Ray {
    pub fn new(A: Vec3, B: Vec3) -> Vec3 {
        Ray(A, B)
    }
    pub fn origin() -> Vec3 {
        A
    }
    pub fn direction() -> Vec3 {
        B
    }
    pub fn point_at_parameter(t: float) -> Vec3 {}
}
