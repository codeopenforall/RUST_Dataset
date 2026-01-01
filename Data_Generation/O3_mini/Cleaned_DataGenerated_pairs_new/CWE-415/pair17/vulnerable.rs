use std::thread;
use std::sync::{Arc, Barrier};
struct Manager {
    ptr: *mut i32,
}
impl Manager {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        Manager {
            ptr: Box::into_raw(boxed),
        }
    }
    fn get(&self) -> i32 {
        unsafe { *self.ptr }
    }
}
impl Drop for Manager {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.ptr);
        }
    }
}
fn main() {
    let manager = Manager::new(500);
    let barrier = Arc::new(Barrier::new(2));
    let dup_ptr = manager.ptr; 
    let barrier_thread = barrier.clone();
    let handler = thread::spawn(move || {
        barrier_thread.wait();
        unsafe {
            Box::from_raw(dup_ptr);
        }
    });
    barrier.wait();
    handler.join().unwrap();
    println!("Value: {}", manager.get());
}
