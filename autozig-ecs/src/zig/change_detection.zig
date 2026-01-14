const std = @import("std");

// 使用extern struct以支持C ABI
pub const Tick = extern struct {
    value: u32,

    pub fn new(value: u32) Tick {
        return Tick{ .value = value };
    }

    pub fn increment(self: *Tick) void {
        self.value +%= 1; // 环绕递增
    }

    pub fn isNewerThan(self: Tick, other: Tick, wrap_threshold: u32) bool {
        // 处理环绕情况的tick比较
        const diff = self.value -% other.value;
        return diff > 0 and diff <= wrap_threshold;
    }
};

pub const ComponentTicks = extern struct {
    added: Tick,
    changed: Tick,

    pub fn new(tick: Tick) ComponentTicks {
        return ComponentTicks{
            .added = tick,
            .changed = tick,
        };
    }

    pub fn setChanged(self: *ComponentTicks, tick: Tick) void {
        self.changed = tick;
    }

    pub fn isAdded(self: ComponentTicks, last_run: Tick, this_run: Tick) bool {
        return self.added.isNewerThan(last_run, this_run.value -% last_run.value);
    }

    pub fn isChanged(self: ComponentTicks, last_run: Tick, this_run: Tick) bool {
        return self.changed.isNewerThan(last_run, this_run.value -% last_run.value);
    }
};

pub const ChangeDetectionContext = extern struct {
    current_tick: Tick,
    last_change_tick: Tick,
    _padding: u64, // 填充以保持ABI兼容

    pub fn init() ChangeDetectionContext {
        return ChangeDetectionContext{
            .current_tick = Tick.new(0),
            .last_change_tick = Tick.new(0),
            ._padding = 0,
        };
    }

    pub fn increment(self: *ChangeDetectionContext) void {
        self.last_change_tick = self.current_tick;
        self.current_tick.increment();
    }

    pub fn checkIfAdded(self: *const ChangeDetectionContext, ticks: ComponentTicks) bool {
        return ticks.isAdded(self.last_change_tick, self.current_tick);
    }

    pub fn checkIfChanged(self: *const ChangeDetectionContext, ticks: ComponentTicks) bool {
        return ticks.isChanged(self.last_change_tick, self.current_tick);
    }
};

// C-compatible exports for FFI
export fn tick_new(value: u32) Tick {
    return Tick.new(value);
}

export fn tick_increment(tick_ptr: *Tick) void {
    tick_ptr.increment();
}

export fn tick_is_newer_than(self: Tick, other: Tick, wrap_threshold: u32) bool {
    return self.isNewerThan(other, wrap_threshold);
}

export fn component_ticks_new(tick: Tick) ComponentTicks {
    return ComponentTicks.new(tick);
}

export fn component_ticks_set_changed(ticks_ptr: *ComponentTicks, tick: Tick) void {
    ticks_ptr.setChanged(tick);
}

export fn component_ticks_is_added(ticks: ComponentTicks, last_run: Tick, this_run: Tick) bool {
    return ticks.isAdded(last_run, this_run);
}

export fn component_ticks_is_changed(ticks: ComponentTicks, last_run: Tick, this_run: Tick) bool {
    return ticks.isChanged(last_run, this_run);
}

export fn change_detection_context_init() ChangeDetectionContext {
    return ChangeDetectionContext.init();
}

export fn change_detection_context_increment(ctx_ptr: *ChangeDetectionContext) void {
    ctx_ptr.increment();
}

export fn change_detection_context_check_if_added(ctx_ptr: *const ChangeDetectionContext, ticks: ComponentTicks) bool {
    return ctx_ptr.checkIfAdded(ticks);
}

export fn change_detection_context_check_if_changed(ctx_ptr: *const ChangeDetectionContext, ticks: ComponentTicks) bool {
    return ctx_ptr.checkIfChanged(ticks);
}
