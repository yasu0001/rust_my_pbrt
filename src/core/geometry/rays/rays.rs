use crate::core::medium::Medium;
use crate::core::geometry::{Point3f, Vector3f};

pub trait BaseRay<'a> {
    fn point(&self, t: f32) -> Point3f{
        self.ray().point(t)
    }
    fn ray(&self) -> &Ray<'a>;
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Ray<'a> {
    pub o: Point3f,
    pub d: Vector3f,
    t_max: f32,
    time: f32,
    medium: Option<&'a Medium>,
}

impl<'a> Ray<'a> {
    pub fn new(o: Point3f, d: Vector3f, t_max: f32, time: f32, medium: Option<&'a Medium>) -> Self {
        Self {
            o, d, t_max, time, medium
        }
    }
}

impl<'a> BaseRay<'a> for Ray<'a> {
    fn point(&self, t: f32) -> Point3f {
        self.o + self.d * t
    }

    fn ray(&self) -> &Self {
        self
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct RayDifferential<'a> {
    ray: Ray<'a>,
    pub has_differentials : bool,
    pub rx_origin : Point3f,
    pub ry_origin : Point3f,
    pub rx_direction : Vector3f,
    pub ry_direction : Vector3f,
}

impl<'a> RayDifferential<'a> {
    pub fn new(o: Point3f, d: Vector3f, t_max: f32, time: f32, medium: Option<&'a Medium>) -> Self {
        let ray = Ray::new(o, d, t_max, time, medium);
        Self::from_ray(ray)
    }

    pub fn from_ray(ray: Ray<'a>) -> Self {
        let rx_origin : Point3f = Default::default();
        let ry_origin : Point3f = Default::default();
        let rx_direction : Vector3f = Default::default();
        let ry_direction : Vector3f = Default::default();
        let has_differentials = false;
        Self {
            ray,
            has_differentials,
            rx_origin,
            ry_origin,
            rx_direction,
            ry_direction,
        }
    }

    pub fn scale_differentials(&mut self, s: f32) {
        self.rx_origin = self.ray.o + (self.rx_origin - self.ray.o) * s;
        self.ry_origin = self.ray.o + (self.ry_origin - self.ray.o) * s;
        self.rx_direction = self.ray.d + (self.rx_direction - self.ray.d) * s;
        self.ry_direction = self.ray.d + (self.ry_direction - self.ray.d) * s;
    }
}

impl<'a> BaseRay<'a> for RayDifferential<'a> {
    fn ray(&self) -> &Ray<'a> {
        &self.ray
    }
}