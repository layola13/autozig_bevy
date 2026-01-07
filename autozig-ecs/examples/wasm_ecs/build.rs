fn main() {
    // Build Zig code for WASM
    autozig_build::build("src").expect("Failed to build Zig code");
}
