use std::sync::atomic::{AtomicUsize, Ordering};
static FREE_COUNT: AtomicUsize = AtomicUsize::new(0);
pub struct Resource {
    data: *mut i32,
}
impl Resource {
    pub unsafe fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Self { data: ptr }
    }
    pub unsafe fn release(&mut self) {
        if !self.data.is_null() {
            FREE_COUNT.fetch_add(1, Ordering::SeqCst);
            let _ = Box::from_raw(self.data);
            self.data = std::ptr::null_mut();
        }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            if !self.data.is_null() {
                FREE_COUNT.fetch_add(1, Ordering::SeqCst);
                let _ = Box::from_raw(self.data);
                self.data = std::ptr::null_mut();
            }
        }
    }
}
fn main() {
    unsafe {
        let mut res = Resource::new(42);
        res.release();
    }
}
