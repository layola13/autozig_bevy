//! 模块 2: Math 示例
//! 演示 autozig-math 的数学运算功能

use autozig_math::*;

pub fn run_math_demo() {
    println!("\n{}", "=".repeat(60));
    println!("模块 2: Math 示例");
    println!("{}", "=".repeat(60));
    
    // Vec2/Vec3/Vec4 示例
    println!("\n[1] 向量运算...");
    let v2 = Vec2::new(1.0, 2.0);
    let v3 = Vec3::new(1.0, 2.0, 3.0);
    let v4 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    
    println!("  Vec2: {:?}", v2);
    println!("  Vec3: {:?}", v3);
    println!("  Vec4: {:?}", v4);
    
    // 整数向量
    println!("\n[2] 整数向量...");
    let iv2 = IVec2::new(1, 2);
    let iv3 = IVec3::new(1, 2, 3);
    let uv2 = UVec2::new(1, 2);
    
    println!("  IVec2: {:?}", iv2);
    println!("  IVec3: {:?}", iv3);
    println!("  UVec2: {:?}", uv2);
    
    // 矩阵运算
    println!("\n[3] 矩阵运算...");
    let m2 = Mat2::IDENTITY;
    let m3 = Mat3::IDENTITY;
    let m4 = Mat4::IDENTITY;
    
    println!("  Mat2: 单位矩阵");
    println!("  Mat3: 单位矩阵");
    println!("  Mat4: 单位矩阵");
    
    // 四元数
    println!("\n[4] 四元数旋转...");
    let q = Quat::IDENTITY;
    println!("  Quat: {:?}", q);
    
    // 几何图元
    println!("\n[5] 几何图元...");
    println!("  - Circle: 圆形");
    println!("  - Sphere: 球体");
    println!("  - Cuboid: 长方体");
    println!("  - Cylinder: 圆柱体");
    println!("  - Capsule: 胶囊体");
    
    // 边界盒
    println!("\n[6] 边界盒...");
    println!("  - Aabb2d: 2D轴对齐边界盒");
    println!("  - Aabb3d: 3D轴对齐边界盒");
    
    // 曲线和样条
    println!("\n[7] 曲线系统...");
    println!("  - CubicBezier3d: 3D贝塞尔曲线");
    println!("  - CubicHermite3d: Hermite样条");
    println!("  - CatmullRom3d: Catmull-Rom样条");
    println!("  - BSpline3d: B样条");
    
    // 方向和旋转
    println!("\n[8] 方向系统...");
    println!("  - Dir2: 2D方向");
    println!("  - Dir3: 3D方向");
    println!("  - Rot2: 2D旋转");
    
    // 等距变换
    println!("\n[9] 变换系统...");
    println!("  - Isometry2d: 2D等距变换");
    println!("  - Isometry3d: 3D等距变换");
    println!("  - Affine2: 2D仿射变换");
    println!("  - Affine3: 3D仿射变换");
    
    // 实用工具
    println!("\n[10] 实用工具...");
    println!("  - AspectRatio: 宽高比");
    println!("  - EaseFunction: 缓动函数");
    println!("  - FloatOrd: 浮点数排序");
    println!("  - Swizzles: 向量分量重排");
    
    println!("\n模块 2 完成 ✓\n");
}