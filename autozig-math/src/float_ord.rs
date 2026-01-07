use autozig::include_zig;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FloatOrd(pub f32);

include_zig!("zig/float_ord.zig", {
    fn float_ord_new(value: f32) -> FloatOrd;
    fn float_ord_cmp(self_: FloatOrd, other: FloatOrd) -> i32;
    fn float_ord_hash(self_: FloatOrd) -> u32;
});

impl FloatOrd {
    pub fn new(value: f32) -> Self {
        float_ord_new(value)
    }
}

impl PartialEq for FloatOrd {
    fn eq(&self, other: &Self) -> bool {
        float_ord_cmp(*self, *other) == 0
    }
}

impl Eq for FloatOrd {}

impl PartialOrd for FloatOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        match float_ord_cmp(*self, *other) {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }
}

impl Hash for FloatOrd {
    fn hash<H: Hasher>(&self, state: &mut H) {
        float_ord_hash(*self).hash(state);
    }
}

impl From<f32> for FloatOrd {
    fn from(value: f32) -> Self {
        FloatOrd::new(value)
    }
}
