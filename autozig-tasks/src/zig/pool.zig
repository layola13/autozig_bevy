const std = @import("std");

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
    mutex: std.Thread.Mutex,
    cond: std.Thread.Condition,

    pub fn init() TaskQueue {
        return TaskQueue{
            .queue = std.ArrayList(Task){},
            .mutex = std.Thread.Mutex{},
            .cond = std.Thread.Condition{},
        };
    }

    pub fn deinit(self: *TaskQueue) void {
        self.queue.deinit(g_allocator);
    }

    pub fn push(self: *TaskQueue, task: Task) !void {
        self.mutex.lock();
        defer self.mutex.unlock();

        try self.queue.append(g_allocator, task);
        self.cond.signal();
    }

    pub fn pop(self: *TaskQueue) ?Task {
        self.mutex.lock();
        defer self.mutex.unlock();

        while (self.queue.items.len == 0) {
            self.cond.wait(&self.mutex);
        }

        return self.queue.orderedRemove(0);
    }

    pub fn tryPop(self: *TaskQueue) ?Task {
        self.mutex.lock();
        defer self.mutex.unlock();

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
    threads: []std.Thread,
    queue: TaskQueue,
    shutdown: std.atomic.Value(bool),
    num_threads: usize,

    pub fn init(num_threads: usize) !*ThreadPool {
        const pool = try g_allocator.create(ThreadPool);
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

        return pool;
    }

    pub fn deinit(self: *ThreadPool) void {
        // 设置shutdown标志
        self.shutdown.store(true, .seq_cst);

        // 唤醒所有worker
        for (0..self.num_threads) |_| {
            self.queue.cond.signal();
        }

        // 等待所有线程结束
        for (self.threads) |thread| {
            thread.join();
        }

        self.queue.deinit();
        g_allocator.free(self.threads);
        g_allocator.destroy(self);
    }

    pub fn submit(self: *ThreadPool, task: Task) !void {
        try self.queue.push(task);
    }

    fn workerLoop(self: *ThreadPool) void {
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
                std.time.sleep(1_000_000); // 1ms
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
