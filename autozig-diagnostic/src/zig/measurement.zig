const std = @import("std");

/// WASM兼容的时间戳获取
fn getTimestamp() i64 {
    // 在 WASM 环境下使用简单的计数器
    // 注意：这不是真实的时间戳，仅用于相对时间测量
    const builtin = @import("builtin");
    if (builtin.target.cpu.arch.isWasm()) {
        // WASM 环境：使用静态计数器
        const State = struct {
            var counter: i64 = 0;
        };
        State.counter += 1;
        return State.counter;
    } else {
        // 非 WASM 环境：使用实际时间戳
        return std.time.nanoTimestamp();
    }
}

/// 诊断测量值
pub const DiagnosticMeasurement = struct {
    value: f64,
    timestamp: i64, // Unix timestamp in nanoseconds

    pub fn create(value: f64) DiagnosticMeasurement {
        return DiagnosticMeasurement{
            .value = value,
            .timestamp = getTimestamp(),
        };
    }

    pub fn createWithTimestamp(value: f64, timestamp: i64) DiagnosticMeasurement {
        return DiagnosticMeasurement{
            .value = value,
            .timestamp = timestamp,
        };
    }
};

// FFI exports
export fn measurement_create(value: f64) DiagnosticMeasurement {
    return DiagnosticMeasurement.create(value);
}

export fn measurement_create_with_timestamp(value: f64, timestamp: i64) DiagnosticMeasurement {
    return DiagnosticMeasurement.createWithTimestamp(value, timestamp);
}

export fn measurement_get_value(measurement: *const DiagnosticMeasurement) f64 {
    return measurement.value;
}

export fn measurement_get_timestamp(measurement: *const DiagnosticMeasurement) i64 {
    return measurement.timestamp;
}
