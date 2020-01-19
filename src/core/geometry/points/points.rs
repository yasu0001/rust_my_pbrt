use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Index};
use crate::core::geometry::{Vector2, Vector3};
use crate::impl_scal_mul;

#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Point2<T> {
    pub x : T,
    pub y : T,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Point3<T> {
    pub x : T,
    pub y : T,
    pub z : T,
}

pub type Point2i = Point2<i16>;
pub type Point2f = Point2<f32>;
pub type Point3i = Point3<i16>;
pub type Point3f = Point3<f32>;

macro_rules! impl_point {
    ($PointN : ident {$($field: ident), *}, $VectorN: ident) => {
        impl<T> $PointN<T> {
            #[inline]
            pub const fn new($($field: T), *) -> Self {
                Self {
                    $($field: $field), *
                }
            }

            pub fn into<U: Into<T> + Copy>(v: &$PointN<U>) -> $PointN<T> {
                $PointN::<T>::new($(v.$field.into()), *)
            }
        }

        impl<T> Add<$VectorN<T>> for $PointN<T> where T: Add<Output=T> {
            type Output = Self;
            fn add(self, v: $VectorN<T>) -> Self {
                Self::new($(self.$field + v.$field), *)
            }
        }

        impl<T> AddAssign<$VectorN<T>> for $PointN<T> where T: AddAssign {
            fn add_assign(&mut self, v: $VectorN<T>) {
                $(self.$field += v.$field);*
            }
        }

        impl<T> Sub<$VectorN<T>> for $PointN<T> where T: Sub<Output=T> {
            type Output=Self;
            fn sub(self, v:$VectorN<T>) -> Self {
                Self::new($(self.$field - v.$field), *)
            }
        }

        impl<T> Sub<$PointN<T>> for $PointN<T> where T: Sub<Output=T> {
            type Output=$VectorN<T>;
            fn sub(self, p: Self) -> $VectorN<T> {
                $VectorN::new($(self.$field - p.$field), *)
            }
        }

        impl<T> SubAssign<$VectorN<T>> for $PointN<T> where T: SubAssign {
            fn sub_assign(&mut self, v: $VectorN<T>) {
                $(self.$field -= v.$field);*
            }
        }

        impl<T> Mul<T> for $PointN<T> where T: Mul<Output=T> + Copy {
            type Output = Self;
            fn mul(self, scaler: T) -> Self {
                Self::new($(self.$field * scaler), *)
            }
        }

        impl<T> MulAssign<T> for $PointN<T> where T: MulAssign + Copy {
            fn mul_assign(&mut self, scaler : T) {
                $(self.$field *= scaler);*
            }
        }

        impl<T> Div<T> for $PointN<T> where T: Div<Output=T> + Mul<Output=T> + Copy {
            type Output = Self;
            fn div(self, scaler: T) -> Self {
                Self::new($(self.$field /scaler), *)
            }
        }

        impl<T> DivAssign<T> for $PointN<T> where T: DivAssign + Copy {
            fn div_assign(&mut self, scaler : T) {
                $(self.$field /= scaler);*
            }
        }

        impl<T> $PointN<T> where T: Add<Output=T> {
            pub fn add_element(p1: Self, p2: Self) -> Self {
                Self::new($(p1.$field + p2.$field), *)
            }
        }

        impl<T> $PointN<T> where T: Sub<Output=T> {
            pub fn sub_element(p1: Self, p2: Self) -> Self {
                Self::new($(p1.$field - p2.$field), *)
            }
        }

        impl<T> $PointN<T> where T: Into<f32> + Copy + Sub<Output=T> {
            pub fn distance(p1: Self, p2: Self) -> f32 {
                (p1 - p2).length()
            }

            pub fn distance_squared(p1: Self, p2: Self) -> f32 {
                (p1 - p2).length_squared()
            }
        }

        impl<T> $PointN<T> where T: Into<f32> + Copy {
            pub fn lerp(t: f32, p1: Self, p2: Self) -> $PointN<f32> {
                $PointN::<f32>::add_element((1.0-t) * $PointN::<f32>::into(&p1), t * $PointN::<f32>::into(&p2))
            }
        }

        impl_scal_mul!($PointN<f32>);
        impl_scal_mul!($PointN<i16>);
    };
}

impl<T> Point2<T> {
    pub fn from_point3(p : Point3<T>) -> Self {
        Point2::new(p.x, p.y)
    }
}

impl<T> Index<usize> for Point2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 3);
        if index == 0 {
            return &self.x;
        }
        &self.y
    }
}

impl<T> Index<usize> for Point3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 3);
        if index == 0 {
            return &self.x;
        }
        if index == 1 {
            return &self.y;
        }
        &self.z
    }
}

impl<T> Point3<T> where T: Copy {
    pub fn permute(v: &Self, x:usize, y:usize, z:usize) -> Self {
        Self::new(v[x], v[y], v[z])
    }
}

impl Point3f {
    pub fn abs(p: &Self) -> Self {
        Self::new(p.x.abs(), p.y.abs(), p.z.abs())
    }

    pub fn ceil(p: &Self) -> Self {
        Self::new(p.x.ceil(), p.y.ceil(), p.z.ceil())
    }

    pub fn floor(p: &Self) -> Self {
        Self::new(p.x.floor(), p.y.floor(), p.z.floor())
    }

    pub fn min(p0: &Self, p1: &Self) -> Self {
        Self::new(p0.x.min(p1.x), p0.y.min(p1.y), p0.z.min(p1.z))
    }

    pub fn max(p0: &Self, p1: &Self) -> Self {
        Self::new(p0.x.max(p1.x), p0.y.max(p1.y), p0.z.max(p1.z))
    }
}

impl_point!(Point2{x, y}, Vector2);
impl_point!(Point3{x, y, z}, Vector3);