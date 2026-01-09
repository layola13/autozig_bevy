use autozig_build::ZigBuilder;

fn main() {
    // 编译Zig代码到WASM目标
    ZigBuilder::new()
        .zig_file("src/scene.zig")
        .zig_file("src/render.zig")
        .zig_file("src/camera.zig")
        .target("wasm32-freestanding")
        .optimize("ReleaseFast")
        .build();
    
    // 告诉Cargo重新编译如果Zig文件改变
    println!("cargo:rerun-if-changed=src/scene.zig");
    println!("cargo:rerun-if-changed=src/render.zig");
    println!("cargo:rerun-if-changed=src/camera.zig");
}