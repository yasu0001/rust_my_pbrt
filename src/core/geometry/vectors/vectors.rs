use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Index};
use crate::impl_scal_mul;

#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Debug)]
pub struct Vector2_F<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Debug)]
pub struct Vector3_F<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

macro_rules! impl_vector {
    ($VectorN: ident {$($field: ident), *}) => {
        impl<T> $VectorN<T> {
            #[inline]
            pub const fn new($($field: T), *) -> Self {
                Self {
                    $($field: $field), *
                }
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
        }
        impl_scal_mul!($VectorN<f32>);
        impl_scal_mul!($VectorN<i16>);
    };
}

impl_vector!(Vector3_F{x, y, z});
impl_vector!(Vector2_F{x, y});