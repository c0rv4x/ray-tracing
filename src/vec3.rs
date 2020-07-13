use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3(f32, f32, f32);

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3(self.0 + _rhs.0, self.1 + _rhs.1, self.2 + _rhs.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3(self.0 - _rhs.0, self.1 - _rhs.1, self.2 - _rhs.2)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3(self * _rhs.0, self * _rhs.1, self * _rhs.2)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3(self.0 * _rhs, self.1 * _rhs, self.2 * _rhs)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f32;
    fn mul(self, _rhs: Vec3) -> f32 {
        self.0 * _rhs.0 + self.1 * _rhs.1 + self.2 * _rhs.2
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f32) -> Vec3 {
        Vec3(self.0 / _rhs, self.1 / _rhs, self.2 / _rhs)
    }
}

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Vec3 {
        Vec3(a, b, c)
    }
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }
    pub fn r(&self) -> f32 {
        self.0
    }
    pub fn g(&self) -> f32 {
        self.1
    }
    pub fn b(&self) -> f32 {
        self.2
    }
    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn print(&self) {
        println!("({}, {}, {})", self.x(), self.y(), self.z());
    }
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}
