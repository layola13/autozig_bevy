fn main() {
    autozig_build::build("src").expect("Failed to build Zig code");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=zig/");
}