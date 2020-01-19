pub use bounds::*;
pub use rays::*;
pub use vectors::*;
pub use points::*;

mod bounds;
mod rays;
mod vectors;
mod points;

#[macro_export]
macro_rules! impl_scal_mul {
    ($VectorN: ident<$type: ty>) => {
        impl Mul<$VectorN<$type>> for $type {
            type Output = $VectorN<$type>;
            #[inline]
            fn mul(self, vec: $VectorN<$type>) -> $VectorN<$type> {
                vec * self
            }
        }
    };
}

#[macro_export]
macro_rules! impl_abs_geometry {
    ($GeometryN: ident<$type: ty> {$($field: ident), *}) => {
        impl $GeometryN<$type> {
            pub fn abs(v: &Self) -> Self {
                $GeometryN::new($(v.$field.abs()), *)
            }
        }
    }
}