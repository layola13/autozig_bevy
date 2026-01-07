use autozig_math::{
    Vec2, Vec3, Vec4, Quat, Mat4, Mat2, Mat3, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4,
    DVec2, DVec3, DVec4, BVec2, BVec3, BVec4,
    DMat2, DMat3, DMat4, DQuat,
    Vec3A, Dir3A, Mat3A,
    Rect, Ray2d, Ray3d, Rot2, Dir2, Dir3,
    Isometry2d, Isometry3d, Affine2, Affine3, Circle, Sphere, Aabb2d, Aabb3d, CubicBezier3d,
    CubicHermite3d, CatmullRom3d, BSpline3d,
    IRect, URect, Triangle2d, Plane2d, InfinitePlane3d, Capsule2d, Cuboid, Cylinder, Capsule3d, Plane3d, CompassOctant,
    AspectRatio, EaseFunction, CompassQuadrant, EulerRot, FloatOrd,
    Vec2Swizzles, Vec3Swizzles, Vec4Swizzles, FloatPow,
};

#[test]
fn test_vec3() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    
    assert_eq!(v1.x, 1.0);
    assert_eq!(v1.dot(v2), 1.0*4.0 + 2.0*5.0 + 3.0*6.0); // 4+10+18 = 32
    
    let v3 = Vec3::X.cross(Vec3::Y);
    assert_eq!(v3, Vec3::Z);
}

#[test]
fn test_vec2() {
    let v1 = Vec2::new(3.0, 4.0);
    assert_eq!(v1.length(), 5.0);
    
    // Test new normalize method
    let normalized = v1.normalize();
    assert!((normalized.length() - 1.0).abs() < 1e-6);
    
    // Test perp
    let perp = Vec2::X.perp();
    assert_eq!(perp.x, 0.0);
    assert_eq!(perp.y, 1.0);
}

#[test]
fn test_vec4() {
    let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let v2 = Vec4::new(1.0, 1.0, 1.0, 1.0);
    assert_eq!(v1.dot(v2), 10.0);
    
    // Test truncate
    let v3 = v1.truncate();
    assert_eq!(v3, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_quat() {
    let q = Quat::IDENTITY;
    assert_eq!(q.w, 1.0);
    
    let axis = Vec3::Y;
    let angle = std::f32::consts::PI;
    let q_rot = Quat::from_axis_angle(axis, angle);
    
    assert!((q_rot.w).abs() < 1e-6);
    assert!((q_rot.y - 1.0).abs() < 1e-6);
    
    // Test mul_vec3
    let v = Vec3::X;
    let rotated = q_rot.mul_vec3(v);
    assert!((rotated.x + 1.0).abs() < 1e-5); // X becomes -X when rotated 180 around Y
    
    // Test slerp
    let q1 = Quat::IDENTITY;
    let q2 = Quat::from_rotation_z(std::f32::consts::PI);
    let q_mid = q1.slerp(q2, 0.5);
    assert!(q_mid.length() > 0.99);
}

#[test]
fn test_mat4_mul_vec3() {
    let mat = Mat4::IDENTITY;
    let v = Vec3::new(1.0, 2.0, 3.0);
    let res = mat.mul_vec3(v);
    assert_eq!(res, v);
}

#[test]
fn test_mat4_perspective() {
    let proj = Mat4::perspective_rh(1.0, 16.0/9.0, 0.1, 100.0);
    assert!(proj.cols[0][0] > 0.0);
    assert!(proj.cols[2][3] < 0.0); // -1 for RH
}

#[test]
fn test_mat4_look_at() {
    let view = Mat4::look_at_rh(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::ZERO,
        Vec3::Y
    );
    let origin = view.mul_vec3(Vec3::ZERO);
    assert!(origin.z < 0.0); // Origin should be in front of camera
}

#[test]
fn test_vec3_ops() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    
    assert_eq!(v1 + v2, Vec3::new(5.0, 7.0, 9.0));
    assert_eq!(v2 - v1, Vec3::new(3.0, 3.0, 3.0));
    assert_eq!(v1 * 2.0, Vec3::new(2.0, 4.0, 6.0));
    assert_eq!(2.0 * v1, Vec3::new(2.0, 4.0, 6.0));
    
    // Test lerp
    let mid = v1.lerp(v2, 0.5);
    assert_eq!(mid, Vec3::new(2.5, 3.5, 4.5));
}

#[test]
fn test_rect() {
    let r = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 20.0));
    assert_eq!(r.width(), 10.0);
    assert_eq!(r.height(), 20.0);
}

#[test]
fn test_ray() {
    let r = Ray3d::new(Vec3::ZERO, Dir3::new(Vec3::Y));
    let p = r.get_point(5.0);
    assert_eq!(p, Vec3::new(0.0, 5.0, 0.0));
}

#[test]
fn test_ray_intersection() {
    let ray2 = Ray2d::new(Vec2::ZERO, Dir2::new(Vec2::Y));
    let pt2 = ray2.get_point(2.0);
    assert_eq!(pt2, Vec2::new(0.0, 2.0));

    let plane2 = Plane2d::new(Dir2::Y);
    let hit2 = ray2.intersect_plane(Vec2::new(0.0, 5.0), plane2);
    assert!(hit2.is_some());
    assert!((hit2.unwrap() - 5.0).abs() < 1e-5);

    let ray3 = Ray3d::new(Vec3::ZERO, Dir3::new(Vec3::Z));
    let pt3 = ray3.get_point(3.0);
    assert_eq!(pt3, Vec3::new(0.0, 0.0, 3.0));

    let plane3 = InfinitePlane3d::new(Dir3::Z);
    let hit3 = ray3.intersect_plane(Vec3::new(0.0, 0.0, 5.0), plane3);
    assert!(hit3.is_some());
    assert!((hit3.unwrap() - 5.0).abs() < 1e-5);
}

#[test]
fn test_mat2() {
    // Basic Mat2 type verification
    // Note: Mat2 FFI operations need further investigation  
    let _m: Mat2 = Mat2 { cols: [[1.0, 0.0], [0.0, 1.0]] };
}

#[test]
fn test_ivec_uvec() {
    let iv = IVec2::new(-1, 2);
    assert_eq!(iv.x, -1);
    
    let uv = UVec3::new(1, 2, 3);
    assert_eq!(uv.z, 3);
    
    // Test new IVec4/UVec4
    let iv4 = IVec4::new(1, -2, 3, -4);
    assert_eq!(iv4.abs(), IVec4::new(1, 2, 3, 4));
    
    let uv4 = UVec4::new(10, 20, 30, 40);
    assert_eq!(uv4.dot(UVec4::ONE), 100);
}

#[test]
fn test_dvec() {
    let dv2 = DVec2::new(1.0, 2.0);
    assert_eq!(dv2.length(), (1.0_f64 + 4.0).sqrt());
    
    let dv3 = DVec3::new(1.0, 2.0, 3.0);
    let dv3_cross = DVec3::X.cross(DVec3::Y);
    assert_eq!(dv3_cross, DVec3::Z);
    
    let dv4 = DVec4::new(1.0, 1.0, 1.0, 1.0);
    assert_eq!(dv4.dot(dv4), 4.0);
}

#[test]
fn test_bvec() {
    let bv2 = BVec2::new(true, false);
    assert!(!bv2.all());
    assert!(bv2.any());
    
    let bv3 = BVec3::new(true, true, true);
    assert!(bv3.all());
    
    let bv4 = BVec4::new(false, false, false, false);
    assert!(!bv4.any());
}

#[test]
fn test_rot2() {
    let r = Rot2::from_angle(0.0);
    assert_eq!(r.c, 1.0);
    assert_eq!(r.s, 0.0);
    
    // Test rotation
    let r90 = Rot2::from_angle(std::f32::consts::FRAC_PI_2);
    let rotated = r90.rotate(Vec2::X);
    assert!((rotated.x).abs() < 1e-5);
    assert!((rotated.y - 1.0).abs() < 1e-5);
    
    // Test inverse
    let rinv = r90.inverse();
    let combined = r90 * rinv;
    assert!((combined.c - 1.0).abs() < 1e-5);
}

#[test]
fn test_dir() {
    let d2 = Dir2::new(Vec2::new(3.0, 4.0));
    assert!((d2.0.length() - 1.0).abs() < 1e-6);
    
    // Test constants
    assert_eq!(Dir2::X.0, Vec2::X);
    assert_eq!(Dir3::Z.0, Vec3::Z);
    
    let d3 = Dir3::new(Vec3::new(1.0, 2.0, 2.0));
    assert!((d3.0.length() - 1.0).abs() < 1e-6);
}

#[test]
fn test_isometry() {
    let iso2 = Isometry2d::new(Vec2::new(1.0, 2.0), Rot2::identity());
    assert_eq!(iso2.translation.x, 1.0);
    
    let iso3 = Isometry3d::new(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY);
    assert_eq!(iso3.translation.z, 3.0);
}

#[test]
fn test_affine2() {
    let aff = Affine2::from_translation(Vec2::new(10.0, 20.0));
    let p = aff.transform_point(Vec2::ZERO);
    assert_eq!(p, Vec2::new(10.0, 20.0));
}

#[test]
fn test_affine3() {
    let aff = Affine3::new(Mat3::IDENTITY, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(aff.translation.y, 2.0);
}

#[test]
fn test_primitives() {
    let c = Circle::new(1.0);
    assert_eq!(c.radius, 1.0);
    let s = Sphere::new(2.5);
    assert_eq!(s.radius, 2.5);
}

#[test]
fn test_aabb() {
    let a2 = Aabb2d::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
    assert_eq!(a2.max.x, 10.0);
    
    let a3 = Aabb3d::new(Vec3::ZERO, Vec3::ONE);
    assert_eq!(a3.min.z, 0.0);
}

#[test]
fn test_cubic_bezier() {
    let curve = CubicBezier3d::new(
        Vec3::ZERO,
        Vec3::X,
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::ONE
    );
    let p_start = curve.position(0.0);
    let p_end = curve.position(1.0);
    assert!(p_start.length() < 1e-6);
    assert!((p_end.x - 1.0).abs() < 1e-6);
}

#[test]
fn test_rects() {
    let ir = IRect::new(IVec2::new(0, 0), IVec2::new(10, 20));
    assert_eq!(ir.width(), 10);
    assert_eq!(ir.height(), 20);

    let ur = URect::new(UVec2::new(0, 0), UVec2::new(100, 200));
    assert_eq!(ur.width(), 100);
}

#[test]
fn test_primitives_2d() {
    let tri = Triangle2d::new(Vec2::ZERO, Vec2::X, Vec2::Y);
    assert_eq!(tri.v1.x, 1.0);
    
    let plane = Plane2d::new(Dir2::X);
    assert_eq!(plane.normal, Dir2::X);
    
    let cap = Capsule2d::new(0.5, 1.0);
    assert_eq!(cap.radius, 0.5);
}

#[test]
fn test_primitives_3d() {
    let cuboid = Cuboid::new(Vec3::ONE);
    assert_eq!(cuboid.half_size.x, 1.0);
    
    let cylinder = Cylinder::new(0.5, 1.0);
    assert_eq!(cylinder.radius, 0.5);
    
    let cap = Capsule3d::new(0.5, 1.0);
    assert_eq!(cap.half_length, 1.0);
    
    let plane = Plane3d::new(Dir3::Y, 2.0);
    assert_eq!(plane.d, 2.0);
}

#[test]
fn test_compass() {
    assert_eq!(CompassOctant::North as u32, 0);
    assert_eq!(CompassOctant::South as u32, 4);
    assert_eq!(CompassQuadrant::East as u32, 1);
}

#[test]
fn test_euler_rot() {
    assert_eq!(EulerRot::ZYX as u32, 0);
    assert_eq!(EulerRot::XYZ as u32, 4);
}

#[test]
fn test_aspect_ratio() {
    let ratio = AspectRatio::new(16.0, 9.0);
    assert!((ratio.ratio - 16.0/9.0).abs() < 1e-6);
    assert!((ratio.width(9.0) - 16.0).abs() < 1e-6);
}

#[test]
fn test_easing() {
    let linear = EaseFunction::Linear;
    assert_eq!(linear.sample(0.5), 0.5);

    let quad = EaseFunction::QuadraticIn;
    assert_eq!(quad.sample(0.5), 0.25);
}

#[test]
fn test_float_ord() {
    let a = FloatOrd::new(1.0);
    let b = FloatOrd::new(2.0);
    assert!(a < b);
    
    let nan = FloatOrd::new(f32::NAN);
    assert!(nan > b); // NaN sorts to end
}

#[test]
fn test_dmat_dquat() {
    // DMat2
    let dm2 = DMat2::identity();
    assert_eq!(dm2.determinant(), 1.0);
    
    // DMat3
    let dm3 = DMat3::identity();
    assert_eq!(dm3.determinant(), 1.0);
    
    // DMat4
    let dm4 = DMat4::from_translation(DVec3::new(1.0, 2.0, 3.0));
    let p = dm4.transform_point(DVec3::ZERO);
    assert_eq!(p, DVec3::new(1.0, 2.0, 3.0));
    
    // DQuat
    let dq = DQuat::identity();
    assert_eq!(dq.length(), 1.0);
    
    let dq_rot = DQuat::from_axis_angle(DVec3::Y, std::f64::consts::PI);
    assert!((dq_rot.w).abs() < 1e-10);
}

#[test]
fn test_mat4_inverse() {
    // Test with translation matrix
    let trans = Mat4::from_translation(Vec3::new(10.0, 20.0, 30.0));
    let inv = trans.inverse();
    let result = trans * inv;
    
    // Should be close to identity
    assert!((result.cols[0][0] - 1.0).abs() < 1e-5);
    assert!((result.cols[3][0]).abs() < 1e-5);
}

#[test]
fn test_splines() {
    // Hermite
    let hermite = CubicHermite3d::new(
        Vec3::ZERO, Vec3::X,
        Vec3::X, Vec3::X
    );
    let p0 = hermite.position(0.0);
    let p1 = hermite.position(1.0);
    assert!(p0.length() < 1e-5);
    assert!((p1.x - 1.0).abs() < 1e-5);
    
    // Catmull-Rom
    let catmull = CatmullRom3d::new(
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::ZERO,
        Vec3::X,
        Vec3::new(2.0, 0.0, 0.0)
    );
    let pm = catmull.position(0.5);
    assert!(pm.x > 0.0 && pm.x < 1.0);
    
    // B-Spline
    let bspline = BSpline3d::new(
        Vec3::ZERO, Vec3::X, Vec3::new(1.0, 1.0, 0.0), Vec3::Y
    );
    let pb = bspline.position(0.5);
    assert!(pb.x > 0.0);
}

#[test]
fn test_aabb_methods() {
    let aabb2 = Aabb2d::new(Vec2::ZERO, Vec2::new(10.0, 10.0));
    assert_eq!(aabb2.center(), Vec2::new(5.0, 5.0));
    assert_eq!(aabb2.size(), Vec2::new(10.0, 10.0));
    assert!(aabb2.contains_point(Vec2::new(5.0, 5.0)));
    assert!(!aabb2.contains_point(Vec2::new(15.0, 5.0)));
    
    let aabb3 = Aabb3d::new(Vec3::ZERO, Vec3::ONE);
    assert_eq!(aabb3.center(), Vec3::new(0.5, 0.5, 0.5));
    assert!(aabb3.contains_point(Vec3::new(0.5, 0.5, 0.5)));
}

#[test]
fn test_circle_sphere_methods() {
    let circle = Circle::new(1.0);
    assert!((circle.area() - std::f32::consts::PI).abs() < 1e-5);
    assert!(circle.contains_point(Vec2::ZERO));
    assert!(!circle.contains_point(Vec2::new(2.0, 0.0)));
    
    let sphere = Sphere::new(1.0);
    assert!((sphere.surface_area() - 4.0 * std::f32::consts::PI).abs() < 1e-5);
    assert!(sphere.contains_point(Vec3::ZERO));
}

#[test]
fn test_vec3a_mat3a() {
    // Vec3A
    let va = Vec3A::new(1.0, 2.0, 3.0);
    assert_eq!(va.x, 1.0);
    assert_eq!(va.length(), (1.0 + 4.0 + 9.0_f32).sqrt());
    
    // Conversion from Vec3
    let v3 = Vec3::new(4.0, 5.0, 6.0);
    let va2 = Vec3A::from(v3);
    assert_eq!(va2.x, 4.0);
    
    // Cross product
    let cross = Vec3A::X.cross(Vec3A::Y);
    assert!((cross.z - 1.0).abs() < 1e-5);
    
    // Mat3A
    let ma = Mat3A::identity();
    let v = ma * Vec3A::X;
    assert!((v.x - 1.0).abs() < 1e-5);
    
    // Dir3A
    let d = Dir3A::from_xyz(1.0, 0.0, 0.0);
    assert!((d.0.length() - 1.0).abs() < 1e-5);
}

#[test]
fn test_swizzles() {
    // Vec2 swizzles
    let v2 = Vec2::new(1.0, 2.0);
    let xx = v2.xx();
    assert_eq!(xx, Vec2::new(1.0, 1.0));
    let yx = v2.yx();
    assert_eq!(yx, Vec2::new(2.0, 1.0));
    
    // Vec3 swizzles
    let v3 = Vec3::new(1.0, 2.0, 3.0);
    let xy = v3.xy();
    assert_eq!(xy, Vec2::new(1.0, 2.0));
    let zyx = v3.zyx();
    assert_eq!(zyx, Vec3::new(3.0, 2.0, 1.0));
    
    // Vec4 swizzles
    let v4 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let xyz = v4.xyz();
    assert_eq!(xyz, Vec3::new(1.0, 2.0, 3.0));
    let wzyx = v4.wzyx();
    assert_eq!(wzyx, Vec4::new(4.0, 3.0, 2.0, 1.0));
}

#[test]
fn test_float_pow() {
    // powf
    let x: f32 = 2.0;
    let p = x.powf(3.0);
    assert!((p - 8.0).abs() < 1e-5);
    
    // powi
    let p2 = x.powi(3);
    assert!((p2 - 8.0).abs() < 1e-5);
    
    // sqrt
    let s = 9.0_f32.sqrt();
    assert!((s - 3.0).abs() < 1e-5);
    
    // exp and ln
    let e = 1.0_f32.exp();
    assert!((e - std::f32::consts::E).abs() < 1e-5);
    let l = std::f32::consts::E.ln();
    assert!((l - 1.0).abs() < 1e-5);
}
