use crate::core::medium::Medium;
use crate::core::geometry::{Point3f, Vector3f};

pub struct Ray<'a> {
    pub o: Point3f,
    pub d: Vector3f,
    t_max: f32,
    time: f32,
    medium: &'a Medium,
}

impl<'a> Ray<'a> {
    pub fn new(o: Point3f, d: Vector3f, t_max: f32, time: f32, medium: &'a Medium) -> Self {
        Self {
            o, d, t_max, time, medium
        }
    }

    pub fn point(&self, t: f32) -> Point3f {
        self.o + self.d * t
    }
}