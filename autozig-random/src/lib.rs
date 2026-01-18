use autozig::include_zig;

include_zig!("src/zig/random.zig", {
    fn random_u64() -> u64;
    fn random_f32() -> f32;
    fn random_bool() -> bool;
});

pub fn get_u64() -> u64 {
    unsafe { random_u64() }
}

pub fn get_f32() -> f32 {
    unsafe { random_f32() }
}

pub fn get_bool() -> bool {
    unsafe { random_bool() }
}
