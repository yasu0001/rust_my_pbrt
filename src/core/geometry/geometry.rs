use std::cmp::{max, min};
use std::convert::From;
use std::f32;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

pub type Vector2f = Vector2<f32>;
pub type Vector2i = Vector2<i16>;
pub type Vector3f = Vector3<f32>;
pub type Vector3i = Vector3<i16>;

pub trait Geometry2<T> {
    type Output;
    fn new(x: T, y: T) -> Self::Output;
}

pub trait Geometry<T> {
    fn new(geo: T) -> Self;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Geometry2<T> for Vector2<T> {
    type Output = Self;
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Vector2<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn dot(v1: &Self, v2: &Self) -> T {
        v1.x * v2.x + v1.y * v2.y
    }
}

impl<T> Vector2<T>
where
    T: Ord + Copy,
{
    pub fn min_component(&self) -> T {
        min(self.x, self.y)
    }

    pub fn max_dimension(&self) -> T {
        max(self.x, self.y)
    }
}

impl<T> Add for Vector2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Vector2<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vector2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> SubAssign for Vector2<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul<T> for Vector2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> MulAssign<T> for Vector2<T>
where
    T: Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> DivAssign<T> for Vector2<T>
where
    T: Div<Output = T> + Copy,
{
    fn div_assign(&mut self, other: T) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> Neg for Vector2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl<T> Index<usize> for Vector2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 3);
        if (index == 0) {
            return &self.x;
        }
        &self.y
    }
}

impl<T> Vector2<T>
where
    T: Into<f32> + Copy,
{
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f32 {
        self.x.into() * self.x.into() + self.y.into() * self.y.into()
    }
}

impl Vector2f {
    pub fn abs(v: &Self) -> Self {
        Self {
            x: v.x.abs(),
            y: v.y.abs(),
        }
    }

    pub fn normalize(v: &Self) -> Self {
        *v / v.length()
    }
}

impl Vector2i {
    pub fn abs(v: &Self) -> Self {
        Self {
            x: v.x.abs(),
            y: v.y.abs(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Vector3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy,
{
    pub fn dot(v1: &Self, v2: &Self) -> T {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

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

impl<T> Vector3<T>
where
    T: Ord + Copy,
{
    pub fn min_component(&self) -> T {
        min(min(self.x, self.y), self.z)
    }

    pub fn max_component(&self) -> T {
        max(max(self.x, self.y), self.z)
    }

    pub fn min(v1: &Self, v2: &Self) -> Self {
        Vector3::new(min(v1.x, v2.x), min(v1.y, v2.y), min(v1.z, v2.z))
    }

    pub fn max(v1: &Self, v2: &Self) -> Self {
        Vector3::new(max(v1.x, v2.x), max(v1.y, v2.y), max(v1.z, v2.z))
    }
}

impl<T> Vector3<T>
where T: PartialOrd + Copy {
    pub fn permute(v: &Self, x: usize, y: usize, z: usize) -> Self {
        Vector3::new(v[x], v[y], v[z])
    }
}

impl<T> Vector3<T>
where
    T: PartialOrd,
{
    pub fn max_dimension(&self) -> i32 {
        if (self.x > self.y) {
            if (self.x > self.z) {
                0
            } else {
                2
            }
        } else {
            if (self.y > self.z) {
                1
            } else {
                2
            }
        }
    }
}

impl<T> Add for Vector3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> AddAssign for Vector3<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vector3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> SubAssign for Vector3<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> MulAssign<T> for Vector3<T>
where
    T: Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Div<Output = T> + Copy,
{
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
where
    T: Div<Output = T> + Copy,
{
    fn div_assign(&mut self, other: T) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> Neg for Vector3<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

impl<T> Index<usize> for Vector3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 4);
        if (index == 0) {
            return &self.x;
        }
        if (index == 1) {
            return &self.y;
        }
        &self.z
    }
}

impl<T> Vector3<T>
where
    T: Into<f32> + Sub<Output = T> + Copy,
{
    pub fn length_squared(&self) -> f32 {
        self.x.into() * self.x.into()
            + self.y.into() * self.y.into()
            + self.z.into() * self.z.into()
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
}

impl Vector3f {
    pub fn abs(v: &Self) -> Self {
        Self {
            x: v.x.abs(),
            y: v.y.abs(),
            z: v.z.abs(),
        }
    }

    pub fn normalize(v: &Self) -> Self {
        *v / v.length()
    }

    pub fn coordinate_sysytem(v1: &Self, v2: &mut Self, v3: &mut Self) {
        if (v1.x.abs() > v1.y.abs()) {
            *v2 = Vector3::new(-v1.z, 0.0, v1.x) / (v1.x * v1.x + v1.z * v1.z).sqrt()
        } else {
            *v2 = Vector3::new(0.0, v1.z, -v1.y) / (v1.y * v1.y + v1.z * v1.z).sqrt()
        }
        *v3 = Vector3f::cross(v1, v2);
    }
}

impl Vector3i {
    pub fn abs(v: &Self) -> Self {
        Self {
            x: v.x.abs(),
            y: v.y.abs(),
            z: v.z.abs(),
        }
    }
}

pub type Point2f = Point2<f32>;
pub type Point2i = Point2<i16>;
pub type Point3f = Point3<f32>;
pub type Point3i = Point3<i16>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn into<U: Into<T> + Copy>(v: &Point2<U>) -> Point2<T> {
        Self {
            x: v.x.into(),
            y: v.y.into(),
        }
    }
}

impl<T> Point2<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn to_vector2(&self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }

    pub fn sub_vector(&self, v: Vector2<T>) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }

    pub fn add_vector(&self, v: Vector2<T>) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl<T> Point2<T>
where
    T: Into<f32> + Sub<Output = T> + Copy,
{
    pub fn length_squared(p1: Self, p2: Self) -> f32 {
        (p1 - p2).length_squared()
    }
}

impl Point2f {
    pub fn length(p1: Self, p2: Self) -> f32 {
        (p1 - p2).length()
    }
}

impl<T> Add<Vector2<T>> for Point2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Vector2<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign<Vector2<T>> for Point2<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Vector2<T>) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Point2<T>
where
    T: Sub<Output = T>,
{
    type Output = Vector2<T>;

    fn sub(self, other: Self) -> Vector2<T> {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl<T> SubAssign<Vector2<T>> for Point2<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, v: Vector2<T>) {
        *self = Self {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl<T> Mul<T> for Point2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> MulAssign<T> for Point2<T>
where
    T: Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl<T> Div<T> for Point2<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> DivAssign<T> for Point2<T>
where
    T: Div<Output = T> + Copy,
{
    fn div_assign(&mut self, other: T) {
        *self = *self / other;
    }
}

impl<T> Index<usize> for Point2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 3);
        if (index == 0) {
            return &self.x;
        }
        &self.y
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    pub fn into<U: Into<T> + Copy>(v: &Point3<U>) -> Point3<T> {
        Self {
            x: v.x.into(),
            y: v.y.into(),
            z: v.z.into(),
        }
    }
}

impl<T> Point3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy,
{
    pub fn to_vector3(&self) -> Vector3<T> {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn sub_vector(&self, v: Vector3<T>) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    pub fn add_vector(&self, v: Vector3<T>) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    pub fn add_point(p0: Self, p1: Self) -> Self {
        Self {
            x: p0.x + p1.x,
            y: p0.y + p1.y,
            z: p0.z + p1.z,
        }
    }
}

impl<T> Point3<T>
    where T: PartialOrd + Copy{
    pub fn permute(v: &Self, x: usize, y: usize, z: usize) -> Self {
        Point3::new(v[x], v[y], v[z])
    }
}

impl<T> Point3<T>
where
    T: Into<f32> + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy,
{
    pub fn lerp(t: f32, p0: Self, p1: Self) -> Point3f {
        Point3f::add_point(Point3f::into(&p0) * (1.0 - t), Point3f::into(&p1) * t)
    }
}

impl<T> Point3<T>
where
    T: Into<f32> + Sub<Output = T> + Copy,
{
    pub fn length_squared(p1: Self, p2: Self) -> f32 {
        (p1 - p2).length_squared()
    }

    pub fn length(p1: Self, p2: Self) -> f32 {
        (p1 - p2).length()
    }

    pub fn distance(p1: &Self, p2: &Self) -> f32 {
        (*p1 - *p2).length()
    }

    pub fn distance_squared(p1: &Self, p2: &Self) -> f32 {
        (*p1 - *p2).length_squared()
    }
}

impl<T> Add<Vector3<T>> for Point3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Vector3<T>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> AddAssign<Vector3<T>> for Point3<T>
where
    T: Add<Output = T> + Copy,
{
    fn add_assign(&mut self, other: Vector3<T>) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Point3<T>
where
    T: Sub<Output = T>,
{
    type Output = Vector3<T>;

    fn sub(self, other: Self) -> Vector3<T> {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T> SubAssign<Vector3<T>> for Point3<T>
where
    T: Sub<Output = T> + Copy,
{
    fn sub_assign(&mut self, v: Vector3<T>) {
        *self = Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl<T> Mul<T> for Point3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> MulAssign<T> for Point3<T>
where
    T: Mul<Output = T> + Copy,
{
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl<T> Div<T> for Point3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> DivAssign<T> for Point3<T>
where
    T: Div<Output = T> + Copy,
{
    fn div_assign(&mut self, other: T) {
        *self = *self / other;
    }
}

impl<T> Neg for Point3<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Index<usize> for Point3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < 3);
        if (index == 0) {
            return &self.x;
        }
        if (index == 1) {
            return &self.y;
        }
        &self.z
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
