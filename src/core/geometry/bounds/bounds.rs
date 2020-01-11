// 領域のソートなどに使用する
// マルチスレッドプログラミングにおいて、サブシーンに分割することで、ここのシーンで処理することが可能になる
use crate::core::geometry::{Point3, Point2, Point3f};
use std::ops::Index;

pub type Bounds2f = Bounds2<f32>;
pub type Bounds2i = Bounds2<i16>;
pub type Bounds3f = Bounds3<f32>;
pub type Bounds3i = Bounds3<i16>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bounds2<T>{
    pub p_min: Point2<T>,
    pub p_max: Point2<T>,
}

impl<T> Bounds2<T> 
where T: Copy {
    pub fn new(min: T, max: T) -> Self {
        Self {p_min: Point2::new(min, min), p_max: Point2::new(max, max)}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bounds3<T>{
    pub p_min: Point3<T>,
    pub p_max: Point3<T>,
}

impl<T> Bounds3<T>
where T: Copy {
    pub fn new(min: T, max: T) -> Self {
        Self {
            p_min: Point3::new(min, min, min),
            p_max: Point3::new(max, max, max),
        }
    }

    pub fn Corner(&self, corner: usize) -> Point3<T> {
        Point3::new(self[corner & 1].x, {
            if corner & 2 == 0 {
                self[1].y
            } else {
                self[0].y
            }
        }, {if corner & 4 == 0 {
                self[1].z
            } else {
                self[0].z
            }
        })
    }
}

impl Bounds3f {
    pub fn union_from_point3f(b: Self, p: Point3f) -> Self {
        Self {
            p_min: Point3f::min(&b.p_min, &p),
            p_max: Point3f::max(&b.p_max, &p),
        }
    }

    pub fn union(b0: Self, b1: Self) -> Self {
        Self {
            p_min: Point3f::min(&b0.p_min, &b1.p_min),
            p_max: Point3f::max(&b0.p_max, &b1.p_max),
        }
    }

    pub fn intersect(b0: Self, b1: Self) -> Self {
        Self {
            p_min: Point3f::max(&b0.p_min, &b1.p_min),
            p_max: Point3f::min(&b0.p_max, &b1.p_max),
        }
    }

    pub fn overlaps(b0: Self, b1: Self) -> bool {
        ((b0.p_max.x >= b1.p_min.x) && (b0.p_min.x <= b1.p_max.x))
        && ((b0.p_max.y >= b1.p_min.y) && (b0.p_min.y <= b1.p_max.y))
        && ((b0.p_max.z >= b1.p_min.z) && (b0.p_min.z <= b1.p_max.z))
    }
}

impl<T> Index<usize> for Bounds3<T> {
    type Output=Point3<T>;
    fn index(&self, ind: usize) -> &Point3<T> {
        if ind == 0 {
            &self.p_min
        } else {
            &self.p_max
        }
    }
}