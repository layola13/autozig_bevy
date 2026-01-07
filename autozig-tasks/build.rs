fn main() {
    // Build Zig code
    autozig_build::build("src").expect("Failed to build Zig code");
}
