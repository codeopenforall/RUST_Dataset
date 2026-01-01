use std::sync::atomic::{AtomicUsize, Ordering};
static GLOBAL_FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
pub struct Resource {
    ptr: *mut i32,
}
unsafe impl Send for Resource {}
impl Resource {
    pub fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Self { ptr }
    }
    pub unsafe fn manual_release(&mut self) {
        if !self.ptr.is_null() {
            let _ = Box::from_raw(self.ptr);
            GLOBAL_FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            self.ptr = std::ptr::null_mut();
        }
    }
    pub unsafe fn get_value(&self) -> Option<i32> {
        if self.ptr.is_null() {
            None
        } else {
            Some(*self.ptr)
        }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                let _ = Box::from_raw(self.ptr);
                GLOBAL_FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
}
pub fn free_count() -> usize {
    GLOBAL_FREE_COUNT.load(Ordering::SeqCst)
}
pub fn run() {
    let mut res = Resource::new(42);
    unsafe {
        res.manual_release();
    }
}
fn main() {
    run();
}
