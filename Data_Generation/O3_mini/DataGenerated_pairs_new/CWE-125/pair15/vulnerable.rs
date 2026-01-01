use std::sync::Arc;
use std::thread;

struct Container {
    buffer: Vec<u32>,
}

impl Container {
    fn compute(&self) -> u32 {
        // The unsafe block performs an unchecked read one element past the end of the buffer.
        unsafe {
            *self.buffer.get_unchecked(self.buffer.len())
        }
    }
}

fn main() {
    let container = Arc::new(Container { buffer: vec![1, 2, 3, 4, 5] });
    let container_clone = Arc::clone(&container);
    let handle = thread::spawn(move || container_clone.compute());
    let res = handle.join().unwrap();
    println!("Result: {}", res);
}