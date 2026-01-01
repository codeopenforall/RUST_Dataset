use std::sync::{Arc, Mutex};
use std::thread;
struct Processor;
impl Processor {
    fn run(&self, req: usize) -> bool {
        let allowed = 5;
        let source = vec![1u8; 10]; 
        let shared_buf = Arc::new(Mutex::new(vec![0u8; 6]));
        {
            let mut buf = shared_buf.lock().unwrap();
            buf[5] = 42; 
        }
        let buf_clone = Arc::clone(&shared_buf);
        let handle = thread::spawn(move || {
            let mut buf = buf_clone.lock().unwrap();
            unsafe {
                std::ptr::copy_nonoverlapping(source.as_ptr(), buf.as_mut_ptr(), req);
            }
        });
        handle.join().unwrap();
        let buf = shared_buf.lock().unwrap();
        buf[5] == 42
    }
}
fn main() {
    let proc = Processor;
    if proc.run(6) {
        println!("Operation successful: Marker intact");
    } else {
        println!("Operation failed: Marker corrupted");
    }
}
