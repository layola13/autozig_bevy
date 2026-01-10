const std = @import("std");
const builtin = @import("builtin");

// 全局allocator
var g_allocator: std.mem.Allocator = std.heap.page_allocator;

// ========== Task定义 ==========

// 任务函数指针类型 (C调用约定)
pub const TaskFn = *const fn (data: *anyopaque) callconv(.c) void;

// Task结构
pub const Task = struct {
    func: TaskFn,
    data: *anyopaque,

    pub fn execute(self: *const Task) void {
        self.func(self.data);
    }
};

// ========== TaskQueue ==========

// 任务队列
pub const TaskQueue = struct {
    queue: std.ArrayList(Task),
    mutex: if (builtin.cpu.arch.isWasm()) void else std.Thread.Mutex,
    cond: if (builtin.cpu.arch.isWasm()) void else std.Thread.Condition,

    pub fn init() TaskQueue {
        return TaskQueue{
            .queue = std.ArrayList(Task){},
            .mutex = if (builtin.cpu.arch.isWasm()) {} else std.Thread.Mutex{},
            .cond = if (builtin.cpu.arch.isWasm()) {} else std.Thread.Condition{},
        };
    }

    pub fn deinit(self: *TaskQueue) void {
        self.queue.deinit(g_allocator);
    }

    pub fn push(self: *TaskQueue, task: Task) !void {
        if (!builtin.cpu.arch.isWasm()) {
            self.mutex.lock();
            defer self.mutex.unlock();
        }

        try self.queue.append(g_allocator, task);

        if (!builtin.cpu.arch.isWasm()) {
            self.cond.signal();
        }
    }

    pub fn pop(self: *TaskQueue) ?Task {
        if (!builtin.cpu.arch.isWasm()) {
            self.mutex.lock();
            defer self.mutex.unlock();

            while (self.queue.items.len == 0) {
                self.cond.wait(&self.mutex);
            }
        } else {
            // WASM: 单线程环境，直接检查队列
            if (self.queue.items.len == 0) {
                return null;
            }
        }

        return self.queue.orderedRemove(0);
    }

    pub fn tryPop(self: *TaskQueue) ?Task {
        if (!builtin.cpu.arch.isWasm()) {
            self.mutex.lock();
            defer self.mutex.unlock();
        }

        if (self.queue.items.len == 0) {
            return null;
        }

        return self.queue.orderedRemove(0);
    }

    pub fn len(self: *const TaskQueue) usize {
        return self.queue.items.len;
    }
};

// ========== ThreadPool ==========

// 线程池
pub const ThreadPool = struct {
    threads: if (builtin.cpu.arch.isWasm()) void else []std.Thread,
    queue: TaskQueue,
    shutdown: std.atomic.Value(bool),
    num_threads: usize,

    pub fn init(num_threads: usize) !*ThreadPool {
        const pool = try g_allocator.create(ThreadPool);

        if (builtin.cpu.arch.isWasm()) {
            // WASM: 单线程环境，不创建线程
            pool.* = ThreadPool{
                .threads = {},
                .queue = TaskQueue.init(),
                .shutdown = std.atomic.Value(bool).init(false),
                .num_threads = 1, // WASM 强制单线程
            };
        } else {
            // Native: 正常多线程环境
            pool.* = ThreadPool{
                .threads = try g_allocator.alloc(std.Thread, num_threads),
                .queue = TaskQueue.init(),
                .shutdown = std.atomic.Value(bool).init(false),
                .num_threads = num_threads,
            };

            // 启动worker线程
            for (pool.threads, 0..) |*thread, i| {
                thread.* = try std.Thread.spawn(.{}, workerLoop, .{pool});
                _ = i;
            }
        }

        return pool;
    }

    pub fn deinit(self: *ThreadPool) void {
        // 设置shutdown标志
        self.shutdown.store(true, .seq_cst);

        if (!builtin.cpu.arch.isWasm()) {
            // 唤醒所有worker
            for (0..self.num_threads) |_| {
                self.queue.cond.signal();
            }

            // 等待所有线程结束
            for (self.threads) |thread| {
                thread.join();
            }

            g_allocator.free(self.threads);
        }

        self.queue.deinit();
        g_allocator.destroy(self);
    }

    pub fn submit(self: *ThreadPool, task: Task) !void {
        try self.queue.push(task);
    }

    pub fn processOne(self: *ThreadPool) bool {
        // WASM专用：手动处理一个任务
        if (self.queue.tryPop()) |task| {
            task.execute();
            return true;
        }
        return false;
    }

    pub fn processAll(self: *ThreadPool) usize {
        // WASM专用：处理所有待处理任务
        var count: usize = 0;
        while (self.processOne()) {
            count += 1;
        }
        return count;
    }

    fn workerLoop(self: *ThreadPool) void {
        // Native专用：worker线程循环
        if (builtin.cpu.arch.isWasm()) {
            return; // WASM不应该调用这个函数
        }

        while (true) {
            // 检查shutdown
            if (self.shutdown.load(.seq_cst)) {
                break;
            }

            // 尝试获取任务
            if (self.queue.tryPop()) |task| {
                task.execute();
            } else {
                // 没有任务，休眠一小会儿避免CPU空转
                std.Thread.sleep(1_000_000); // 1ms
            }
        }
    }
};

// ========== Exported C API ==========

// Task API
export fn task_execute(func: TaskFn, data: *anyopaque) void {
    const task = Task{
        .func = func,
        .data = data,
    };
    task.execute();
}

// ThreadPool API
export fn thread_pool_create(num_threads: usize) *ThreadPool {
    return ThreadPool.init(num_threads) catch unreachable;
}

export fn thread_pool_destroy(pool: *ThreadPool) void {
    pool.deinit();
}

export fn thread_pool_submit(pool: *ThreadPool, func: TaskFn, data: *anyopaque) bool {
    const task = Task{
        .func = func,
        .data = data,
    };
    pool.submit(task) catch return false;
    return true;
}

export fn thread_pool_num_threads(pool: *const ThreadPool) usize {
    return pool.num_threads;
}

export fn thread_pool_pending_tasks(pool: *const ThreadPool) usize {
    return pool.queue.len();
}

// WASM专用API：手动处理任务
export fn thread_pool_process_one(pool: *ThreadPool) bool {
    return pool.processOne();
}

export fn thread_pool_process_all(pool: *ThreadPool) usize {
    return pool.processAll();
}
