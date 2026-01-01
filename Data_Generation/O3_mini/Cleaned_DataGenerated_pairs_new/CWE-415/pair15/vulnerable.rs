#![allow(unused)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
trait Operations {
    unsafe fn release(&self);
}
pub struct Manager {
    ptr: *mut i32,
}
impl Manager {
    pub unsafe fn initialize(value: i32) -> Self {
        let boxed = Box::new(value);
        Manager {
            ptr: Box::into_raw(boxed),
        }
    }
    pub unsafe fn process(&self) {
        self.release();
    }
}
static DEALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
impl Operations for Manager {
    unsafe fn release(&self) {
        let count = DEALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
        if count >= 1 {
            panic!("Double free detected during explicit release (unsafe manual deallocation)!");
        }
        let _ = Box::from_raw(self.ptr);
    }
}
impl Drop for Manager {
    fn drop(&mut self) {
        unsafe {
            let count = DEALLOC_COUNT.fetch_add(1, Ordering::SeqCst);
            if count >= 1 {
                panic!("Double free detected in destructor (Drop)!");
            }
            let _ = Box::from_raw(self.ptr);
        }
    }
}
pub fn simulate() {
    unsafe {
        let resource = Manager::initialize(100);
        resource.process();
    }
}
fn main() {
    simulate();
}
