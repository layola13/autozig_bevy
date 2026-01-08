const std = @import("std");

/// 诊断测量值
pub const DiagnosticMeasurement = struct {
    value: f64,
    timestamp: i64, // Unix timestamp in nanoseconds

    pub fn create(value: f64) DiagnosticMeasurement {
        return DiagnosticMeasurement{
            .value = value,
            .timestamp = std.time.nanoTimestamp(),
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
