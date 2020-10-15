pub struct HitableList {
    values: Vec<Hitable>,
}

impl Hitable for HitableList {
    fn hits(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let closest_so_far: f32 = tmax;
    }
}
