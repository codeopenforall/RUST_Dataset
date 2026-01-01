use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
static FREED: AtomicUsize = AtomicUsize::new(0);
unsafe fn custom_release(ptr: *mut i32) {
    if FREED.fetch_add(1, Ordering::SeqCst) >= 1 {
        panic!("Double free detected");
    }
    drop(Box::from_raw(ptr));
}
struct Resource {
    ptr: *mut i32,
}
unsafe impl Send for Resource {}
unsafe impl Sync for Resource {}
impl Resource {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        let ptr = Box::into_raw(boxed);
        Resource { ptr }
    }
    fn release(&self) {
        unsafe {
            custom_release(self.ptr);
        }
    }
}
impl Drop for Resource {
    fn drop(&mut self) {
        unsafe {
            custom_release(self.ptr);
        }
    }
}
fn main() {
    let res = Arc::new(Resource::new(42));
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        res_clone.release();
    });
    handle.join().unwrap();
}
