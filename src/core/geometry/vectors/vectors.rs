use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Index};
use std::cmp::{min, max};
use std::f32;
use crate::{impl_scal_mul, impl_abs_geometry};

#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Debug, Default)]
pub struct Normal3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vector2f = Vector2<f32>;
pub type Vector2i = Vector2<i16>;
pub type Vector3f = Vector3<f32>;
pub type Vector3i = Vector3<i16>;
pub type Normal3f = Normal3<f32>;

macro_rules! impl_vector {
    ($VectorN: ident {$($field: ident), *}) => {
        impl<T> $VectorN<T> {
            #[inline]
            pub const fn new($($field: T), *) -> Self {
                Self {
                    $($field: $field), *
                }
            }
            pub fn into<U: Into<T> + Copy>(v: &$VectorN<U>) -> $VectorN<T> {
                $VectorN::<T>::new($(v.$field.into()), *)
            }
        }

        impl<T> Add for $VectorN<T> where T: Add<Output=T> {
            type Output=Self;
            fn add(self, other: Self) -> Self {
                $VectorN::new($(self.$field + other.$field), *)
            }
        }

        impl<T> AddAssign for $VectorN<T> where T:AddAssign {
            fn add_assign(&mut self, other: Self) {
                $(self.$field += other.$field); *
            }
        }

        impl<T> Sub for $VectorN<T> where T: Sub<Output=T> {
            type Output=Self;
            fn sub(self, other: Self) -> Self {
                $VectorN::new($(self.$field - other.$field), *)
            }
        }

        impl<T> SubAssign for $VectorN<T> where T: SubAssign {
            fn sub_assign(&mut self, other: Self) {
                $(self.$field -= other.$field); *
            }
        }

        impl<T> Mul<T> for $VectorN<T> where T: Mul<Output=T> + Copy {
            type Output = Self;
            fn mul(self, other: T) -> Self {
                Self::new($(self.$field * other), *)
            }
        }

        impl<T> MulAssign<T> for $VectorN<T> where T: MulAssign + Copy {
            fn mul_assign(&mut self, other: T) {
                $(self.$field *= other); *
            }
        }

        impl<T> Div<T> for $VectorN<T> where T: Div<Output=T> + Copy {
            type Output = Self;

            fn div(self, other: T) -> Self {
                Self::new($(self.$field / other), *)
            }
        }

        impl<T> DivAssign<T> for $VectorN<T> where T: DivAssign + Copy {
            fn div_assign(&mut self, other: T) {
                $(self.$field /= other);*
            }
        }

        impl<T> Neg for $VectorN<T> where T: Neg<Output=T>{
            type Output = Self;

            fn neg(self) -> Self {
                Self::new($(self.$field.neg()), *)
            }
        }

        impl<T> $VectorN<T> where T: Into<f32> + Copy {
            pub fn length(&self) -> f32 {
                self.length_squared().sqrt()

            }
            pub fn length_squared(&self) -> f32 {
                let mut length_sq = 0.0;
                $(length_sq += self.$field.into() * self.$field.into());*;
                length_sq
            }

            pub fn normalize(v: &Self) -> $VectorN<f32> {
                $VectorN::<f32>::into(v) / v.length()
            }
        }

        impl<T> $VectorN<T> where T: Into<f32> + Copy {

        }

        impl_scal_mul!($VectorN<f32>);
        impl_scal_mul!($VectorN<i16>);

        impl_abs_geometry!($VectorN<f32> {$($field), *});
        impl_abs_geometry!($VectorN<i16> {$($field), *});
        
    };
}

macro_rules! impl_abs_dot_vector {
    ($VectorN: ident<$type: ty>) => {
        impl $VectorN<$type> {
            #[inline]
            pub fn abs_dot(v1: &Self, v2: &Self) -> $type {
                Self::dot(v1, v2).abs()
            }
        }
    }
}

impl<T> Index<usize> for Vector2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 3);
        if index == 0 {
            &self.x
        } else {
            &self.y
        }
    }
}

impl<T> Index<usize> for Vector3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 4);
        if index == 0 {
            &self.x
        } else if index == 1{
            &self.y
        } else {
            &self.z
        }
    }
}

impl<T> Index<usize> for Normal3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 4);
        if index == 0 {
            &self.x
        } else if index == 1{
            &self.y
        } else {
            &self.z
        }
    }
}

impl<T> Vector3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy,
{
    #[inline]
    pub fn dot(v1: &Self, v2: &Self) -> T {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    #[inline]
    pub fn cross(v1: &Self, v2: &Self) -> Self {
        let v1x = v1.x;
        let v1y = v1.y;
        let v1z = v1.z;
        let v2x = v2.x;
        let v2y = v2.y;
        let v2z = v2.z;
        Vector3::new(
            v1x * v2z - v1z * v2y,
            v1z * v2x - v1x * v2z,
            v1x * v2y - v1y * v2x,
        )
    }
}

impl<T> Vector3<T>  where T: PartialOrd {
    pub fn max_dimension(v: &Self) -> usize {
        if v.x > v.y {
            if v.x > v.z {
                0
            } else {
                1
            }
        } else {
            if v.y > v.z {
                1
            } else {
                2
            }
        }
    }
}

impl<T> Vector3<T> where T: Copy {
    pub fn permute(v: Self, x:usize, y:usize, z:usize) -> Self {
        Vector3::new(v[x], v[y], v[z])
    }
}

impl Vector3<f32> {
    pub fn min_component(v: &Self) -> f32 {
        v.x.min(v.y.min(v.z))
    }

    pub fn max_component(v: &Self) -> f32 {
        v.x.max(v.y.max(v.z))
    }
    // TODO
    pub fn coordinate_sysytem(v1: &Self, v2: &mut Self, v3: &mut Self) {
        if v1.x.abs() > v1.y.abs() {
            *v2 = Vector3::new(-v1.z, 0.0, v1.x) / (v1.x * v1.x + v1.z * v1.z).sqrt()
        } else {
            *v2 = Vector3::new(0.0, v1.z, -v1.y) / (v1.y * v1.y + v1.z * v1.z).sqrt()
        }
        *v3 = Vector3::<f32>::cross(v1, v2);
    }
}

impl Vector3<i16> where {
    pub fn min_component(v: &Self) -> i16 {
        min(v.x, min(v.y, v.z))
    }

    pub fn max_component(v: &Self) -> i16 {
        max(v.x, max(v.y, v.z))
    }
}

impl<T> Normal3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy,
{
    #[inline]
    pub fn dot(v1: &Self, v2: &Self) -> T {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    #[inline]
    pub fn cross(v1: &Self, v2: &Self) -> Self {
        let v1x = v1.x;
        let v1y = v1.y;
        let v1z = v1.z;
        let v2x = v2.x;
        let v2y = v2.y;
        let v2z = v2.z;
        Self::new(
            v1x * v2z - v1z * v2y,
            v1z * v2x - v1x * v2z,
            v1x * v2y - v1y * v2x,
        )
    }
}

impl_abs_dot_vector!(Vector3<f32>);
impl_abs_dot_vector!(Vector3<i16>);
impl_abs_dot_vector!(Normal3<f32>);
impl_abs_dot_vector!(Normal3<i16>);
impl_vector!(Vector3{x, y, z});
impl_vector!(Vector2{x, y});
impl_vector!(Normal3{x, y, z});
