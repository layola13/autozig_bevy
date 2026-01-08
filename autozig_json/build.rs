fn main() {
    // Use modular buildzig mode to avoid file merge conflicts
    std::env::set_var("AUTOZIG_MODE", "modular_buildzig");
    
    // Build Zig JSON parser
    autozig_build::build("src").expect("Failed to build Zig JSON parser");
}
