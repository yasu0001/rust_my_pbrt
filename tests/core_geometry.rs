mod core_geometry_tests {
    use rust_my_pbrt::core::geometry::*;
    #[test]
    
    // test arithmetic operation for vector2
    fn check_arith_vector2() {
        let a = Vector2::new(1.0, 3.0);
        let mut ma = Vector2::new(1.0, 3.0);
        let b = Vector2::new(1.0, 3.0);
        let scal = 2.0;
        let scal2 = -1.0;

        let ab = Vector2::new(2.0, 6.0);
        let amb = Vector2::new(0.0, 0.0);
        let amuls = Vector2::new(2.0, 6.0);
        let adivs = Vector2::new(1.0 / 2.0, 3.0 / 2.0);
        assert_eq!(1.0, a[0]);
        assert_eq!(3.0, a[1]);
        assert_eq!(ab, a + b);
        assert_eq!(amb, a - b);
        assert_eq!(amuls, a * scal);
        assert_eq!(adivs, a / scal);
        assert_eq!(-a, a * scal2);
        ma += b;
        assert_eq!(ab, ma);

        ma = a;
        ma -= b;
        assert_eq!(amb, ma);

        ma = a;
        ma *= scal;
        assert_eq!(amuls, ma);

        ma = a;
        ma /= scal;

        assert_eq!(adivs, ma);

        let c = Vector2::new(1.0, -3.0);
        assert_eq!(Vector2f::abs(&c), Vector2::new(c.x.abs(), c.y.abs()));
    }

    #[test]
    fn check_arith_vector3() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let mut ma = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(1.0, 3.0, 4.0);
        let scal = 2.0;
        let scal2 = -1.0;

        let ab = Vector3::new(a.x + b.x, a.y + b.y, a.z + b.z);
        let amb = Vector3::new(a.x - b.x, a.y - b.y, a.z - b.z);
        let amuls = Vector3::new(a.x * scal, a.y * scal, a.z * scal);
        let adivs = Vector3::new(a.x / scal, a.y / scal, a.z / scal);
        assert_eq!(1.0, a[0]);
        assert_eq!(2.0, a[1]);
        assert_eq!(ab, a + b);
        assert_eq!(amb, a - b);
        assert_eq!(amuls, a * scal);
        assert_eq!(adivs, a / scal);
        assert_eq!(-a, a * scal2);
        ma += b;
        assert_eq!(ab, ma);
        ma = a;
        ma -= b;
        assert_eq!(amb, ma);

        ma = a;
        ma *= scal;
        assert_eq!(amuls, ma);

        ma = a;
        ma /= scal;

        assert_eq!(adivs, ma);

        let c = Vector3f::new(1.0, -3.0, 2.0);
        assert_eq!(
            Vector3f::abs(&c),
            Vector3f::new(c.x.abs(), c.y.abs(), c.z.abs())
        );
    }

    #[test]
    fn check_arith_point2() {
        // test for new
        assert_eq!(Point2 { x: 1.0, y: 2.0 }, Point2::new(1.0, 2.0));

        let a = Point2i::new(1, 2);
        assert_eq!(Point2f::new(1.0, 2.0), Point2f::into(&a));
        assert_eq!(Vector2::new(1, 2), a.to_vector2());

        let a = Point2::new(1.0, 2.0);
        let c = Point2::new(3.0, 1.0);
        let b = Vector2::new(2.0, 3.0);
        let scal = 3.0;
        assert_eq!(Point2::new(a.x + b.x, a.y + b.y), a + b);
        assert_eq!(Vector2::new(a.x - c.x, a.y - c.y), a - c);
        assert_eq!(Point2::new(a.x * scal, a.y * scal), a * scal);
        assert_eq!(Point2::new(a.x / scal, a.y / scal), a / scal);
        assert_eq!(1.0, a[0]);
        assert_eq!(2.0, a[1]);

        let mut ma = Point2::new(1.0, 2.0);
        ma += b;
        assert_eq!(ma, a + b);
        ma = a;
        ma -= b;
        assert_eq!(ma, a.sub_vector(b));

        ma = a;
        ma *= scal;
        assert_eq!(ma, a * scal);

        ma = a;
        ma /= scal;
        assert_eq!(ma, a / scal);

        let a = Point2f::new(1.0, 2.0);
        let b = Point2f::new(1.0, 3.0);
        assert_eq!(Point2f::length(a, b), (a - b).length());
        assert_eq!(Point2f::length_squared(a, b), (a - b).length_squared());
    }

    #[test]
    fn check_arith_point3() {
        assert_eq!(
            Point3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            },
            Point3::new(1.0, 2.0, 3.0)
        );

        let a = Point3i::new(1, 2, 3);
        assert_eq!(Vector3i::new(1, 2, 3), a.to_vector3());

        let a = Point3::new(1.0, 2.0, 3.0);
        let c = Point3::new(3.0, 1.0, 4.0);
        let b = Vector3::new(2.0, 3.0, 1.0);
        let scal = 3.0;
        assert_eq!(Point3::new(a.x + b.x, a.y + b.y, a.z + b.z), a + b);
        assert_eq!(Vector3::new(a.x - c.x, a.y - c.y, a.z - c.z), a - c);
        assert_eq!(Point3::new(a.x * scal, a.y * scal, a.z * scal), a * scal);
        assert_eq!(Point3::new(a.x / scal, a.y / scal, a.z / scal), a / scal);
        assert_eq!(1.0, a[0]);
        assert_eq!(2.0, a[1]);

        let mut ma = Point3::new(1.0, 2.0, 3.0);
        ma += b;
        assert_eq!(ma, a + b);

        ma = a;
        ma -= b;
        assert_eq!(ma, a.sub_vector(b));

        ma = a;
        ma *= scal;
        assert_eq!(ma, a * scal);

        ma = a;
        ma /= scal;
        assert_eq!(ma, a / scal);

        let a = Point3f::new(1.2, 2.6, 4.0);
        let b = Point3f::new(1.0, -3.0, 3.0);
        let t: f32 = 0.7;
        assert_eq!(Point3f::length(a, b), (a - b).length());
        assert_eq!(Point3f::length_squared(a, b), (a - b).length_squared());

        assert_eq!(Point3f::new(a.x * (1.0-t) + b.x * t, a.y * (1.0-t) + b.y * t,
        a.z * (1.0-t) + b.z * t), Point3f::lerp(t, a, b));

        assert_eq!(Point3f::ceil(&a), Point3f::new(a.x.ceil(), a.y.ceil(), a.z.ceil()));
        assert_eq!(Point3f::floor(&a), Point3f::new(a.x.floor(), a.y.floor(), a.z.floor()));
        assert_eq!(Point3f::abs(&a), Point3f::new(a.x.abs(), a.y.abs(), a.z.abs()));
        //assert_eq!(Point3f::max(&a, &b), Point3f::new())
        assert_eq!(Point3f::max(&a, &b), Point3f::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z)));
        assert_eq!(Point3f::min(&a, &b), Point3f::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z)));
        assert_eq!(Point3f::permute(&a, 1, 2, 0), Point3f::new(a.y, a.z, a.x));
    }

    #[test]
    fn check_arith_normal() {
        assert_eq!(
            Normal3 {
                x: 1.0,
                y: 2.0,
                z: 3.0
            },
            Normal3::new(1.0, 2.0, 3.0)
        );

        let a = Normal3::new(1, 2, 3);
        let a = Normal3::new(1.0, 2.0, 3.0);
        let c = Normal3::new(3.0, 1.0, 4.0);
        let b = Normal3::new(2.0, 3.0, 1.0);
        let scal = 3.0;
        assert_eq!(Normal3::new(a.x + b.x, a.y + b.y, a.z + b.z), a + b);
        assert_eq!(Normal3::new(a.x - c.x, a.y - c.y, a.z - c.z), a - c);
        assert_eq!(Normal3::new(a.x * scal, a.y * scal, a.z * scal), a * scal);
        assert_eq!(Normal3::new(a.x / scal, a.y / scal, a.z / scal), a / scal);
        assert_eq!(1.0, a[0]);
        assert_eq!(2.0, a[1]);

        let mut ma = Normal3::new(1.0, 2.0, 3.0);
        ma += b;
        assert_eq!(ma, a + b);

        ma = a;
        ma *= scal;
        assert_eq!(ma, a * scal);

        ma = a;
        ma /= scal;
        assert_eq!(ma, a / scal);

        let a = Normal3::new(1.2, 2.6, 4.0);
        let b = Normal3::new(1.0, -3.0, 3.0);
        let t: f32 = 0.7;
        assert_eq!(a.length_squared(), a.x * a.x + a.y * a.y + a.z + a.z as f32);
        assert_eq!(a.length_squared().sqrt(), a.length());
    }
}
