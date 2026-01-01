use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr;
static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);
struct Handler {
    raw: *mut i32,
}
impl Handler {
    unsafe fn create(num: i32) -> Self {
        let boxed = Box::new(num);
        Self {
            raw: Box::into_raw(boxed),
        }
    }
    unsafe fn dispose(&mut self) {
        if !self.raw.is_null() {
            if GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                panic!("Double free detected in dispose");
            }
            let _ = Box::from_raw(self.raw);
            self.raw = ptr::null_mut(); 
        }
    }
}
impl Drop for Handler {
    fn drop(&mut self) {
        unsafe {
            if !self.raw.is_null() {
                if GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst) >= 1 {
                    panic!("Double free detected in Drop");
                }
                let _ = Box::from_raw(self.raw);
            }
        }
    }
}
fn main() {
    unsafe {
        let mut obj = Handler::create(200);
        obj.dispose(); 
    }
    println!("Execution completed (corrected path).");
}
