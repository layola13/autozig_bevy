const std = @import("std");
const hashmap = @import("hashmap.zig");

// 使用hashmap.zig中定义的allocator
const g_allocator = hashmap.g_allocator;

// 原子计数器 (WASM32 兼容：使用 u32 而非 u64)
pub const AtomicCounter = struct {
    value: std.atomic.Value(u32),

    pub fn init(initial: u32) AtomicCounter {
        return AtomicCounter{
            .value = std.atomic.Value(u32).init(initial),
        };
    }

    pub fn load(self: *AtomicCounter, comptime order: std.builtin.AtomicOrder) u32 {
        return self.value.load(order);
    }

    pub fn store(self: *AtomicCounter, val: u32, comptime order: std.builtin.AtomicOrder) void {
        self.value.store(val, order);
    }

    pub fn fetchAdd(self: *AtomicCounter, delta: u32, comptime order: std.builtin.AtomicOrder) u32 {
        return self.value.fetchAdd(delta, order);
    }

    pub fn fetchSub(self: *AtomicCounter, delta: u32, comptime order: std.builtin.AtomicOrder) u32 {
        return self.value.fetchSub(delta, order);
    }

    pub fn increment(self: *AtomicCounter) u32 {
        return self.fetchAdd(1, .seq_cst);
    }

    pub fn decrement(self: *AtomicCounter) u32 {
        return self.fetchSub(1, .seq_cst);
    }

    pub fn compareAndSwap(
        self: *AtomicCounter,
        expected: u32,
        new: u32,
        comptime success_order: std.builtin.AtomicOrder,
        comptime failure_order: std.builtin.AtomicOrder,
    ) ?u32 {
        return self.value.cmpxchgWeak(expected, new, success_order, failure_order);
    }
};

// 原子布尔值
pub const AtomicBool = struct {
    value: std.atomic.Value(bool),

    pub fn init(initial: bool) AtomicBool {
        return AtomicBool{
            .value = std.atomic.Value(bool).init(initial),
        };
    }

    pub fn load(self: *AtomicBool, comptime order: std.builtin.AtomicOrder) bool {
        return self.value.load(order);
    }

    pub fn store(self: *AtomicBool, val: bool, comptime order: std.builtin.AtomicOrder) void {
        self.value.store(val, order);
    }

    pub fn swap(self: *AtomicBool, val: bool, comptime order: std.builtin.AtomicOrder) bool {
        return self.value.swap(val, order);
    }
};

// 自旋锁 (WASM环境下的简化实现)
pub const SpinLock = struct {
    locked: std.atomic.Value(bool),

    pub fn init() SpinLock {
        return SpinLock{
            .locked = std.atomic.Value(bool).init(false),
        };
    }

    pub fn lock(self: *SpinLock) void {
        while (self.locked.swap(true, .acquire)) {
            // 在WASM单线程环境下，这个循环不会执行
            // 但保留实现以保持API一致性
            std.atomic.spinLoopHint();
        }
    }

    pub fn tryLock(self: *SpinLock) bool {
        return !self.locked.swap(true, .acquire);
    }

    pub fn unlock(self: *SpinLock) void {
        self.locked.store(false, .release);
    }

    pub fn isLocked(self: *SpinLock) bool {
        return self.locked.load(.acquire);
    }
};

// 一次性初始化标记
pub const OnceFlag = struct {
    state: std.atomic.Value(u8),

    const State = struct {
        const UNINITIALIZED: u8 = 0;
        const INITIALIZING: u8 = 1;
        const INITIALIZED: u8 = 2;
    };

    pub fn init() OnceFlag {
        return OnceFlag{
            .state = std.atomic.Value(u8).init(State.UNINITIALIZED),
        };
    }

    pub fn callOnce(self: *OnceFlag, comptime func: fn () void) void {
        if (self.state.load(.acquire) == State.INITIALIZED) {
            return;
        }

        const prev = self.state.cmpxchgStrong(
            State.UNINITIALIZED,
            State.INITIALIZING,
            .acquire,
            .acquire,
        );

        if (prev == null) {
            // 我们是第一个，执行初始化
            func();
            self.state.store(State.INITIALIZED, .release);
        } else {
            // 等待其他线程完成初始化
            while (self.state.load(.acquire) != State.INITIALIZED) {
                std.atomic.spinLoopHint();
            }
        }
    }

    pub fn isInitialized(self: *OnceFlag) bool {
        return self.state.load(.acquire) == State.INITIALIZED;
    }

    pub fn reset(self: *OnceFlag) void {
        self.state.store(State.UNINITIALIZED, .release);
    }
};

// FFI导出 - AtomicCounter (WASM32: u32)
export fn atomic_counter_create(initial: u32) *AtomicCounter {
    const counter = g_allocator.create(AtomicCounter) catch unreachable;
    counter.* = AtomicCounter.init(initial);
    return counter;
}

export fn atomic_counter_destroy(counter: *AtomicCounter) void {
    g_allocator.destroy(counter);
}

export fn atomic_counter_load(counter: *AtomicCounter) u32 {
    return counter.load(.seq_cst);
}

export fn atomic_counter_store(counter: *AtomicCounter, value: u32) void {
    counter.store(value, .seq_cst);
}

export fn atomic_counter_increment(counter: *AtomicCounter) u32 {
    return counter.increment();
}

export fn atomic_counter_decrement(counter: *AtomicCounter) u32 {
    return counter.decrement();
}

export fn atomic_counter_add(counter: *AtomicCounter, delta: u32) u32 {
    return counter.fetchAdd(delta, .seq_cst);
}

export fn atomic_counter_sub(counter: *AtomicCounter, delta: u32) u32 {
    return counter.fetchSub(delta, .seq_cst);
}

// FFI导出 - AtomicBool
export fn atomic_bool_create(initial: bool) *AtomicBool {
    const atomic_bool = g_allocator.create(AtomicBool) catch unreachable;
    atomic_bool.* = AtomicBool.init(initial);
    return atomic_bool;
}

export fn atomic_bool_destroy(atomic_bool: *AtomicBool) void {
    g_allocator.destroy(atomic_bool);
}

export fn atomic_bool_load(atomic_bool: *AtomicBool) bool {
    return atomic_bool.load(.seq_cst);
}

export fn atomic_bool_store(atomic_bool: *AtomicBool, value: bool) void {
    atomic_bool.store(value, .seq_cst);
}

export fn atomic_bool_swap(atomic_bool: *AtomicBool, value: bool) bool {
    return atomic_bool.swap(value, .seq_cst);
}

// FFI导出 - SpinLock
export fn spinlock_create() *SpinLock {
    const lock = g_allocator.create(SpinLock) catch unreachable;
    lock.* = SpinLock.init();
    return lock;
}

export fn spinlock_destroy(lock: *SpinLock) void {
    g_allocator.destroy(lock);
}

export fn spinlock_lock(lock: *SpinLock) void {
    lock.lock();
}

export fn spinlock_try_lock(lock: *SpinLock) bool {
    return lock.tryLock();
}

export fn spinlock_unlock(lock: *SpinLock) void {
    lock.unlock();
}

export fn spinlock_is_locked(lock: *SpinLock) bool {
    return lock.isLocked();
}

// FFI导出 - OnceFlag
export fn once_flag_create() *OnceFlag {
    const flag = g_allocator.create(OnceFlag) catch unreachable;
    flag.* = OnceFlag.init();
    return flag;
}

export fn once_flag_destroy(flag: *OnceFlag) void {
    g_allocator.destroy(flag);
}

export fn once_flag_is_initialized(flag: *OnceFlag) bool {
    return flag.isInitialized();
}

export fn once_flag_reset(flag: *OnceFlag) void {
    flag.reset();
}

// 单元测试
test "AtomicCounter basic operations" {
    var counter = AtomicCounter.init(0);

    try std.testing.expectEqual(@as(u32, 0), counter.load(.seq_cst));

    counter.store(10, .seq_cst);
    try std.testing.expectEqual(@as(u32, 10), counter.load(.seq_cst));

    const old = counter.increment();
    try std.testing.expectEqual(@as(u32, 10), old);
    try std.testing.expectEqual(@as(u32, 11), counter.load(.seq_cst));

    _ = counter.decrement();
    try std.testing.expectEqual(@as(u32, 10), counter.load(.seq_cst));
}

test "AtomicCounter add/sub" {
    var counter = AtomicCounter.init(100);

    const old_add = counter.fetchAdd(50, .seq_cst);
    try std.testing.expectEqual(@as(u32, 100), old_add);
    try std.testing.expectEqual(@as(u32, 150), counter.load(.seq_cst));

    const old_sub = counter.fetchSub(30, .seq_cst);
    try std.testing.expectEqual(@as(u32, 150), old_sub);
    try std.testing.expectEqual(@as(u32, 120), counter.load(.seq_cst));
}

test "AtomicBool operations" {
    var atomic_bool = AtomicBool.init(false);

    try std.testing.expectEqual(false, atomic_bool.load(.seq_cst));

    atomic_bool.store(true, .seq_cst);
    try std.testing.expectEqual(true, atomic_bool.load(.seq_cst));

    const old = atomic_bool.swap(false, .seq_cst);
    try std.testing.expectEqual(true, old);
    try std.testing.expectEqual(false, atomic_bool.load(.seq_cst));
}

test "SpinLock operations" {
    var lock = SpinLock.init();

    try std.testing.expect(!lock.isLocked());

    try std.testing.expect(lock.tryLock());
    try std.testing.expect(lock.isLocked());

    try std.testing.expect(!lock.tryLock());

    lock.unlock();
    try std.testing.expect(!lock.isLocked());
}

// 测试OnceFlag - 使用全局变量避免闭包捕获问题
var test_once_counter: u32 = 0;

test "OnceFlag initialization" {
    var flag = OnceFlag.init();
    test_once_counter = 0;

    try std.testing.expect(!flag.isInitialized());

    // 第一次调用应该执行
    flag.callOnce(struct {
        fn init() void {
            test_once_counter += 1;
        }
    }.init);
    try std.testing.expect(flag.isInitialized());
    try std.testing.expectEqual(@as(u32, 1), test_once_counter);

    // 第二次调用不应该执行
    flag.callOnce(struct {
        fn init() void {
            test_once_counter += 1;
        }
    }.init);
    try std.testing.expectEqual(@as(u32, 1), test_once_counter);
}
