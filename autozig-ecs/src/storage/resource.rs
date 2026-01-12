//! Resource storage implementation

use crate::component::ComponentId;
use crate::change_detection::{Tick, ComponentTicks};
use std::ptr::NonNull;

/// ResourceData - 单个资源的数据和元数据
pub struct ResourceData {
    data: Option<NonNull<u8>>,
    ticks: ComponentTicks,
    is_present: bool,
}

impl ResourceData {
    pub fn new() -> Self {
        Self {
            data: None,
            ticks: ComponentTicks::new(Tick::new(0)),
            is_present: false,
        }
    }
    
    pub fn is_present(&self) -> bool {
        self.is_present
    }
    
    pub fn get_ticks(&self) -> Option<ComponentTicks> {
        if self.is_present {
            Some(self.ticks)
        } else {
            None
        }
    }
    
    // TODO: 实现剩余的resource API
}

impl Default for ResourceData {
    fn default() -> Self {
        Self::new()
    }
}

/// Resources - 资源存储集合
#[derive(Default)]
pub struct Resources {
    resources: std::collections::HashMap<ComponentId, ResourceData>,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn clear(&mut self) {
        self.resources.clear();
    }
    
    pub fn get(&self, id: ComponentId) -> Option<&ResourceData> {
        self.resources.get(&id)
    }
    
    pub fn get_mut(&mut self, id: ComponentId) -> Option<&mut ResourceData> {
        self.resources.get_mut(&id)
    }
}