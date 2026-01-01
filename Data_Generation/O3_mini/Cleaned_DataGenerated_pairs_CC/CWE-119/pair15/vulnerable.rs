use std::sync::{Arc, Mutex};
use std::thread;
trait Operations {
    fn update(&self, index: usize, value: i32);
}
struct Buffer {
    data: Mutex<Vec<i32>>,
}
impl Buffer {
    fn new(size: usize) -> Self {
        Self {
            data: Mutex::new(vec![0; size]),
        }
    }
}
impl Operations for Buffer {
    fn update(&self, index: usize, value: i32) {
        let mut guard = self.data.lock().unwrap();
        unsafe {
            let ptr = guard.as_mut_ptr();
            *ptr.offset(index as isize) = value;
        }
    }
}
fn main() {
    let buf = Arc::new(Buffer::new(10));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let b = Arc::clone(&buf);
            thread::spawn(move || {
                b.update(15, 99);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let guard = buf.data.lock().unwrap();
    println!("Buffer contents: {:?}", *guard);
}
