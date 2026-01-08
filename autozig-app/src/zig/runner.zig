const std = @import("std");
const ZigApp = @import("app.zig").ZigApp;

/// Default runner - runs the app once
pub fn defaultRunner(app: *ZigApp) u8 {
    app.update();
    return if (app.exit_code) |code| code else 0;
}

/// Loop runner - runs the app in a loop until exit
pub fn loopRunner(app: *ZigApp) u8 {
    while (app.shouldExit() < 0) {
        app.update();
    }
    return if (app.exit_code) |code| code else 0;
}

/// Fixed timestep runner - runs with fixed timestep
pub fn fixedTimestepRunner(app: *ZigApp) u8 {
    const fixed_dt: f64 = 1.0 / 60.0; // 60 FPS
    var accumulator: f64 = 0.0;
    var last_time = std.time.milliTimestamp();

    while (app.shouldExit() < 0) {
        const current_time = std.time.milliTimestamp();
        const frame_time = @as(f64, @floatFromInt(current_time - last_time)) / 1000.0;
        last_time = current_time;

        accumulator += frame_time;

        while (accumulator >= fixed_dt) {
            app.update();
            accumulator -= fixed_dt;
        }
    }

    return if (app.exit_code) |code| code else 0;
}

/// Frame limiter runner - limits frame rate
pub fn frameLimitRunner(app: *ZigApp, target_fps: u32) u8 {
    const frame_time_ms = 1000 / target_fps;

    while (app.shouldExit() < 0) {
        const start_time = std.time.milliTimestamp();

        app.update();

        const elapsed = std.time.milliTimestamp() - start_time;
        if (elapsed < frame_time_ms) {
            std.time.sleep(@intCast((frame_time_ms - elapsed) * 1_000_000));
        }
    }

    return if (app.exit_code) |code| code else 0;
}

// FFI exports for common runners
export fn runner_default(app: *ZigApp) u8 {
    return defaultRunner(app);
}

export fn runner_loop(app: *ZigApp) u8 {
    return loopRunner(app);
}

export fn runner_fixed_timestep(app: *ZigApp) u8 {
    return fixedTimestepRunner(app);
}
