use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::thread;
struct Data {
    value: u32,
}
struct Holder {
    ptr: *mut Data,
}
impl Holder {
    fn new(val: u32) -> Self {
        let boxed = Box::new(Data { value: val });
        Self { ptr: Box::into_raw(boxed) }
    }
    unsafe fn read(&self) -> u32 {
        (*self.ptr).value
    }
}
impl Drop for Holder {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                Box::from_raw(self.ptr);
            }
        }
    }
}
static GLOBAL_PTR: AtomicPtr<Data> = AtomicPtr::new(ptr::null_mut());
pub fn run_op() -> u32 {
    let holder = Holder::new(100);
    GLOBAL_PTR.store(holder.ptr, Ordering::SeqCst);
    drop(holder);
    let handle = thread::spawn(|| unsafe {
        let p = GLOBAL_PTR.load(Ordering::SeqCst);
        (*p).value
    });
    let result = handle.join().unwrap();
    result
}
fn main() {
    let res = run_op();
    println!("Result: {}", res);
}
