pub use geometry::*;
pub use bounds::*;
pub use rays::*;
pub use vectors::*;

mod geometry;
mod bounds;
mod rays;
mod vectors;

#[macro_export]
macro_rules! impl_scal_mul {
    ($VectorN: ident<$type: ty>) => {
        impl Mul<$VectorN<$type>> for $type {
            type Output = $VectorN<$type>;
            fn mul(self, vec: $VectorN<$type>) -> $VectorN<$type> {
                vec * self
            }
        }
    };
}