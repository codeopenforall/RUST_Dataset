use std::sync::atomic::{AtomicBool, Ordering};
static FREED: AtomicBool = AtomicBool::new(false);
struct Data {
    ptr: *mut i32,
impl Data {
        Data { ptr: Box::into_raw(boxed) }
impl Drop for Data {
        unsafe {
            if !FREED.compare_and_swap(false, true, Ordering::SeqCst) {
                let _ = Box::from_raw(self.ptr);
            } else {
                let _ = Box::from_raw(self.ptr);
                panic!("Double free detected");
    let instance1 = Data::new(100);
    let _instance2 = Data { ptr: instance1.ptr };
    100
