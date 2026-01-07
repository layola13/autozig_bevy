//! AutoZig Tasks - 轻量级任务系统
//! 
//! 90% Zig实现的线程池系统，提供Bevy兼容的API

use autozig::include_zig;

#[repr(C)]
pub struct ThreadPoolOpaque {
    _private: u8,
}

pub type TaskFn = extern "C" fn(*mut std::ffi::c_void);

include_zig!("src/zig/pool.zig", {
    fn thread_pool_create(num_threads: usize) -> *mut ThreadPoolOpaque;
    fn thread_pool_destroy(pool: *mut ThreadPoolOpaque);
    fn thread_pool_submit(pool: *mut ThreadPoolOpaque, func: TaskFn, data: *mut std::ffi::c_void) -> bool;
    fn thread_pool_num_threads(pool: *const ThreadPoolOpaque) -> usize;
    fn thread_pool_pending_tasks(pool: *const ThreadPoolOpaque) -> usize;
});

/// 任务池核心
pub struct TaskPool {
    inner: *mut ThreadPoolOpaque,
}

impl TaskPool {
    /// 创建一个新的任务池
    pub fn new() -> Self {
        let num_cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        Self::with_num_threads(num_cpus)
    }
    
    /// 创建指定线程数的任务池
    pub fn with_num_threads(num_threads: usize) -> Self {
        Self {
            inner: thread_pool_create(num_threads),
        }
    }
    
    /// 提交一个任务
    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 将闭包装箱并转换为C函数指针
        let boxed = Box::new(f);
        let data = Box::into_raw(boxed) as *mut std::ffi::c_void;
        
        extern "C" fn run_closure<F: FnOnce()>(data: *mut std::ffi::c_void) {
            let boxed = unsafe { Box::from_raw(data as *mut F) };
            boxed();
        }
        
        thread_pool_submit(self.inner, run_closure::<F>, data);
    }
    
    /// 获取线程数
    pub fn thread_num(&self) -> usize {
        thread_pool_num_threads(self.inner)
    }
    
    /// 获取待处理任务数
    pub fn queued_tasks(&self) -> usize {
        thread_pool_pending_tasks(self.inner)
    }
}

impl Drop for TaskPool {
    fn drop(&mut self) {
        thread_pool_destroy(self.inner);
    }
}

impl Default for TaskPool {
    fn default() -> Self {
        Self::new()
    }
}

pub mod prelude {
    pub use super::TaskPool;
}
