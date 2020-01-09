mod core_geometry_tests {
    use rust_my_pbrt::core::geometry::*;

    #[test]
    // test arithmetic operation for vector2
    fn check_arith_vector2() {
        let a = Vector2::new(1.0, 2.0);
        let mut ma = Vector2::new(1.0, 2.0);
        let b = Vector2::new(1.0, 3.0);
        let scal = 2.0;
        let scal2 = -1.0;

        let ab = Vector2::new(2.0, 5.0);
        let amb = Vector2::new(0.0, -1.0);
        let amuls = Vector2::new(2.0, 4.0);
        let adivs = Vector2::new(1.0/2.0, 2.0/2.0);
        assert_eq!(1.0, a[0]);
        assert_eq!(2.0, a[1]);
        assert_eq!(ab, a+b);
        assert_eq!(amb, a-b);
        assert_eq!(amuls, a*scal);
        assert_eq!(adivs, a/scal);
        assert_eq!(-a, a*scal2);
        ma += b;
        assert_eq!(ab, ma);
        ma = a;
        ma -= b;
        assert_eq!(amb, ma);

        ma=a;
        ma*=scal;
        assert_eq!(amuls, ma);

        ma = a;
        ma /= scal;

        assert_eq!(adivs, ma);

        let c = Vector2f::new(1.0, -3.0);
        assert_eq!(Vector2f::abs(&c), Vector2f::new(c.x.abs(), c.y.abs()));
    }

    #[test]
    fn check_arith_vector3() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let mut ma = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(1.0, 3.0, 4.0);
        let scal = 2.0;
        let scal2 = -1.0;

        let ab = Vector3::new(a.x+b.x, a.y+b.y, a.z+b.z);
        let amb = Vector3::new(a.x-b.x, a.y-b.y, a.z-b.z);
        let amuls = Vector3::new(a.x*scal, a.y*scal, a.z*scal);
        let adivs = Vector3::new(a.x/scal, a.y/scal, a.z/scal);
        assert_eq!(1.0, a[0]);
        assert_eq!(2.0, a[1]);
        assert_eq!(ab, a+b);
        assert_eq!(amb, a-b);
        assert_eq!(amuls, a*scal);
        assert_eq!(adivs, a/scal);
        assert_eq!(-a, a*scal2);
        ma += b;
        assert_eq!(ab, ma);
        ma = a;
        ma -= b;
        assert_eq!(amb, ma);

        ma=a;
        ma*=scal;
        assert_eq!(amuls, ma);

        ma = a;
        ma /= scal;

        assert_eq!(adivs, ma);

        let c = Vector3f::new(1.0, -3.0, 2.0);
        assert_eq!(Vector3f::abs(&c), Vector3f::new(c.x.abs(), c.y.abs(), c.z.abs()));
    }
}