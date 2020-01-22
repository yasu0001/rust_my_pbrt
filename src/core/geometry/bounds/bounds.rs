use crate::core::geometry::{Point2, Point3, Point3f, Vector3};
use std::ops::{Add, Index, Sub};

pub type Bounds2f = Bounds2<f32>;
pub type Bounds2i = Bounds2<i16>;
pub type Bounds3f = Bounds3<f32>;
pub type Bounds3i = Bounds3<i16>;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Bounds2<T> {
    pub p_min: Point2<T>,
    pub p_max: Point2<T>,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Bounds3<T> {
    pub p_min: Point3<T>,
    pub p_max: Point3<T>,
}

macro_rules! impl_bounds {
    ($BoundsN : ident {$field1: ident, $field2: ident}, $PointN:ident {$($field: ident), *}, $VectorN:ident {$($fieldv : ident), *}) => {
        impl<T> $BoundsN<T> {
            #[inline]
            pub const fn new($field1: $PointN<T>, $field2: $PointN<T>) -> Self {
                Self {
                    $field1, $field2
                }
            }
        }
        impl<T> $BoundsN<T> where T : Copy {
            #[inline]
            pub fn from_single_point(p: $PointN<T>) -> Self {
                Self::new(p, p)
            }
        }

        impl<T> $BoundsN<T> where T: PartialOrd {
            pub fn overlaps(b1: &Self, b2: &Self) -> bool {
                let mut is_overlap = true;
                $(is_overlap = is_overlap && b1.$field2.$field >= b2.$field1.$field && b1.$field1.$field <= b2.$field2.$field;)*
                is_overlap
            }

            pub fn inside(p: &$PointN<T>, b: &Self) -> bool {
                let mut is_inside = true;
                $(is_inside = is_inside && p.$field >= b.$field1.$field && p.$field<= b.$field2.$field;)*
                is_inside
            }

            pub fn inside_exclusive(p: $PointN<T>, b: Self) -> bool {
                let mut is_inside = true;
                $(is_inside = is_inside && p.$field >= b.$field1.$field && p.$field< b.$field2.$field;)*
                is_inside
            }
        }

        impl<T> $BoundsN<T> where T : Sub<Output=T> +  Add<Output=T> + Copy {
            pub fn expand(b : &Self, delta: T) -> Self {
                Self::new(b.$field1 - $VectorN{$($field: delta), *}, b.$field2 + $VectorN{$($field: delta), *})
            }
        }

        impl<T> $BoundsN<T> where T : Sub<Output=T> + Copy {
            pub fn diagonal(&self) -> $VectorN<T> {
                self.$field2 - self.$field1
            }
        }

        impl_bounds_set_op!($BoundsN<f32>{$field1, $field2}, $PointN);

    }
}

macro_rules! impl_bounds_set_op {
    ($BoundsN : ident<$T: ty> {$field1: ident, $field2: ident}, $PointN: ident) => {
        impl $BoundsN<$T> {
            pub fn union_from_point(b: &Self, p: $PointN<$T>) -> Self {
                Self::new(
                    $PointN::<$T>::min(&b.$field1, &p),
                    $PointN::<$T>::max(&b.$field2, &p),
                )
            }

            pub fn union(b1: &Self, b2: &Self) -> Self {
                Self::new(
                    $PointN::min(&b1.$field1, &b2.$field1),
                    $PointN::max(&b1.$field2, &b2.$field2),
                )
            }

            pub fn intersect(b1: &Self, b2: &Self) -> Self {
                Self::new(
                    $PointN::max(&b1.$field1, &b2.$field1),
                    $PointN::min(&b1.$field2, &b2.$field2),
                )
            }
        }
    };
}

impl_bounds!(
    Bounds3 { p_min, p_max },
    Point3 { x, y, z },
    Vector3 { x, y, z }
);
