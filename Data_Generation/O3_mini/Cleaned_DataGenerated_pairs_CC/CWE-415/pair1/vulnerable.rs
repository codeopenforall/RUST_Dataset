use std::sync::atomic::{AtomicBool, Ordering};
static FREED: AtomicBool = AtomicBool::new(false);
struct Data {
    ptr: *mut i32,
}
impl Data {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        Data { ptr: Box::into_raw(boxed) }
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            if !FREED.compare_and_swap(false, true, Ordering::SeqCst) {
                let _ = Box::from_raw(self.ptr);
            } else {
                let _ = Box::from_raw(self.ptr);
                panic!("Double free detected");
            }
        }
    }
}
fn run_app() -> i32 {
    let instance1 = Data::new(100);
    let _instance2 = Data { ptr: instance1.ptr };
    100
}
fn main() {
    let _ = run_app();
}
