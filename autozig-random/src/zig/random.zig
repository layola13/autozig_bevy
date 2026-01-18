const std = @import("std");

var prng: std.rand.DefaultPrng = undefined;
var random: std.rand.Random = undefined;
var initialized: bool = false;

fn init() void {
    if (!initialized) {
        var seed: u64 = undefined;
        std.crypto.random.bytes(std.mem.asBytes(&seed));
        prng = std.rand.DefaultPrng.init(seed);
        random = prng.random();
        initialized = true;
    }
}

export fn random_u64() u64 {
    if (!initialized) init();
    return random.int(u64);
}

export fn random_f32() f32 {
    if (!initialized) init();
    return random.float(f32);
}

export fn random_bool() bool {
    if (!initialized) init();
    return random.boolean();
}
