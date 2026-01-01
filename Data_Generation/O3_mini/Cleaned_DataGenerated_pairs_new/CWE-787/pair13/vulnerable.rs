use std::thread;
use std::sync::{Arc, Mutex};
struct Data {
    buf: Vec<u8>,
}
impl Data {
    pub fn new(size: usize) -> Self {
        Data {
            buf: Vec::with_capacity(size),
        }
    }
    pub unsafe fn load(&mut self, input: &[u8]) {
        let capacity = self.buf.capacity();
        let count = input.len();
        let ptr = self.buf.as_mut_ptr();
        for i in 0..count {
            *ptr.add(i) = input[i];
        }
        self.buf.set_len(count);
    }
    pub fn read(&self) -> &[u8] {
        &self.buf
    }
}
fn run(input: Vec<u8>) -> Result<Arc<Mutex<Data>>, &'static str> {
    let shared = Arc::new(Mutex::new(Data::new(10)));
    let shared_clone = Arc::clone(&shared);
    let handler = thread::spawn(move || {
        let mut data = shared_clone.lock().unwrap();
        unsafe {
            data.load(&input);
        }
    });
    if handler.join().is_err() {
        return Err("Thread panicked");
    }
    Ok(shared)
}
fn main() {
    let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let res = run(input);
    match res {
        Ok(shared) => {
            let data = shared.lock().unwrap();
            println!("Buffer content: {:?}", data.read());
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
