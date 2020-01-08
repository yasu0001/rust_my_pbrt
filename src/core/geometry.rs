use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Index};

pub type Vector2f=Vector2<f32>;
pub type Vector2i=Vector2<i32>;
pub type Vector3f=Vector3<f32>;
pub type Vector3i=Vector3<i32>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T> Vector2<T> 
    where T: Mul<Output=T> + Add<Output=T> + Copy {
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl<T> Add for Vector2<T>
    where T: Add<Output=T> {
    type Output=Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T> AddAssign for Vector2<T>
    where T: Add<Output=T> + Copy {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T> Sub for Vector2<T> 
    where T: Sub<Output=T> {
    type Output=Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Vector2<T> 
    where T: Sub<Output=T> + Copy {
    fn sub_assign(&mut self, other: Self){
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul<T> for Vector2<T>
    where T: Mul<Output=T> + Copy {
    type Output=Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> MulAssign<T> for Vector2<T>
    where T: Mul<Output=T> + Copy {
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> Div<T> for Vector2<T>
    where T: Div<Output=T> + Copy {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> DivAssign<T> for Vector2<T>
    where T: Div<Output=T> + Copy {
    fn div_assign(&mut self, other: T) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> Neg for Vector2<T>
    where T: Neg<Output=T> {
    type Output=Self;

    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl<T> Index<usize> for Vector2<T> {
    type Output=T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 3);
        if (index == 0) {return &self.x;}
        &self.y
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {x, y, z}
    }
}

impl<T> Vector3<T> 
    where T: Mul<Output=T> + Add<Output=T> + Copy {
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T> Add for Vector3<T>
    where T: Add<Output=T> {
    type Output=Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> AddAssign for Vector3<T>
    where T: Add<Output=T> + Copy {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vector3<T> 
    where T: Sub<Output=T> {
    type Output=Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> SubAssign for Vector3<T> 
    where T: Sub<Output=T> + Copy {
    fn sub_assign(&mut self, other: Self){
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<T> for Vector3<T>
    where T: Mul<Output=T> + Copy {
    type Output=Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> MulAssign<T> for Vector3<T>
    where T: Mul<Output=T> + Copy {
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> Div<T> for Vector3<T>
    where T: Div<Output=T> + Copy {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> DivAssign<T> for Vector3<T>
    where T: Div<Output=T> + Copy {
    fn div_assign(&mut self, other: T) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> Neg for Vector3<T>
    where T: Neg<Output=T> {
    type Output=Self;

    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

impl<T> Index<usize> for Vector3<T> {
    type Output=T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 4);
        if (index == 0) {return &self.x;}
        if (index == 1) {return &self.y;}
        &self.z
    }
}