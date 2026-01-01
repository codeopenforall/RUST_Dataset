use std::alloc::{alloc, dealloc, Layout};
use std::ptr::copy_nonoverlapping;
use std::sync::{Arc, Mutex};
use std::thread;
struct DataHandler {
    data: String,
}
impl DataHandler {
    unsafe fn duplicate(&self) -> *mut u8 {
        let len = self.data.len();
        let layout = Layout::from_size_align(len + 1, 1).unwrap();
        let buffer = alloc(layout);
        copy_nonoverlapping(self.data.as_ptr(), buffer, len);
        *buffer.add(len) = 0;
        buffer
    }
}
impl Drop for DataHandler {
    fn drop(&mut self) {
    }
}
trait Execution {
    fn execute(&self, trigger: &str) -> bool;
}
struct ConcurrentWorker;
impl Execution for ConcurrentWorker {
    fn execute(&self, trigger: &str) -> bool {
        let handler = Arc::new(Mutex::new(DataHandler {
            data: trigger.to_owned(),
        }));
        let mut handles = vec![];
        for _ in 0..4 {
            let handler_clone = Arc::clone(&handler);
            let t = thread::spawn(move || {
                let data = handler_clone.lock().unwrap();
                unsafe {
                    let ptr = data.duplicate();
                    let len = data.data.len();
                    let layout = Layout::from_size_align(len + 1, 1).unwrap();
                    dealloc(ptr, layout);
                }
            });
            handles.push(t);
        }
        for h in handles {
            h.join().expect("Thread panicked");
        }
        true
    }
}
fn main() {
    let trigger = "overflow";
    let worker = ConcurrentWorker;
    if worker.execute(trigger) {
        println!("Success");
    } else {
        println!("Failure");
    }
}
