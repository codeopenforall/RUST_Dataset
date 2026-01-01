use std::sync::Arc;
use std::thread;
struct Buffer {
    data: Box<[i32]>,
}
impl Buffer {
    fn new(v: Vec<i32>) -> Self {
        Buffer { data: v.into_boxed_slice() }
    }
    fn access(&self, idx: usize) -> Option<i32> {
        unsafe {
            Some(*self.data.get_unchecked(idx))
        }
    }
}
fn execute(buffer: Arc<Buffer>, idx: usize) -> Option<i32> {
    buffer.access(idx)
}
fn main() {
    let buffer = Arc::new(Buffer::new((0..10).collect()));
    let idx = 15; 
    let handle = {
        let buf = Arc::clone(&buffer);
        thread::spawn(move || {
            execute(buf, idx)
        })
    };
    match handle.join().unwrap() {
        Some(val) => println!("Result: {}", val),
        None => println!("None"),
    }
}
