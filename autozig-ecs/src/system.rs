// System placeholder - to be implemented

use autozig::include_zig;

#[repr(C)]
pub struct ScheduleOpaque {
    _private: u8,
}

// System function pointer type matching Zig
pub type SystemFn = extern "C" fn(*mut std::ffi::c_void);

include_zig!("src/zig/system.zig", {
    fn schedule_create() -> *mut ScheduleOpaque;
    fn schedule_destroy(schedule: *mut ScheduleOpaque);
    fn schedule_add_system(schedule: *mut ScheduleOpaque, name_ptr: *const u8, name_len: usize, func: SystemFn) -> bool;
    fn schedule_run(schedule: *mut ScheduleOpaque, world_ptr: *mut std::ffi::c_void);
    fn schedule_system_count(schedule: *const ScheduleOpaque) -> usize;
});

pub struct Schedule {
    inner: *mut ScheduleOpaque,
}

impl Schedule {
    pub fn new() -> Self {
        let inner = schedule_create();
        Self { inner }
    }
    
    pub fn add_system(&mut self, name: &str, func: SystemFn) -> bool {
        schedule_add_system(self.inner, name.as_ptr(), name.len(), func)
    }
    
    pub fn run(&mut self, world_ptr: *mut std::ffi::c_void) {
        schedule_run(self.inner, world_ptr);
    }
    
    pub fn system_count(&self) -> usize {
        schedule_system_count(self.inner)
    }
}

impl Drop for Schedule {
    fn drop(&mut self) {
        schedule_destroy(self.inner);
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self::new()
    }
}

// System trait for user-defined systems
pub trait System: Send + Sync {
    fn run(&mut self, world: &mut crate::world::World);
}

