// Rust implementation of atomic.d
// Provides atomic field operations

use std::sync::atomic::{AtomicI32, Ordering};

#[repr(C)]
pub struct AtomicField {
    pub field: AtomicI32,
}

impl AtomicField {
    pub fn new(value: i32) -> Self {
        AtomicField {
            field: AtomicI32::new(value),
        }
    }

    pub fn load(&self) -> i32 {
        self.field.load(Ordering::SeqCst)
    }

    pub fn store(&self, value: i32) {
        self.field.store(value, Ordering::SeqCst);
    }

    pub fn fetch_add(&self, delta: i32) -> i32 {
        self.field.fetch_add(delta, Ordering::SeqCst)
    }

    pub fn test(&self) -> bool {
        self.load() != 0
    }
}

pub fn compiler_barrier() {
    std::sync::atomic::compiler_fence(Ordering::SeqCst);
}
