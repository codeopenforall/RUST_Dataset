use std::sync::{Arc, Mutex};
struct MemoryBuffer {
    buffer: Vec<u8>,
    capacity: usize,
}
impl MemoryBuffer {
    fn new(capacity: usize, guard: u8) -> Self {
        let mut vec = Vec::with_capacity(capacity + 1);
        unsafe {
            vec.set_len(capacity + 1);
        }
        vec[capacity] = guard;
        Self {
            buffer: vec,
            capacity,
        }
    }
    fn write_input(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.capacity {
            return Err("Input length exceeds buffer capacity");
        }
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            for i in 0..input.len() {
                *ptr.add(i) = input[i];
            }
            if input.len() > self.buffer.len() {
                self.buffer.set_len(input.len());
            }
        }
        Ok(())
    }
    fn check_guard(&self, guard: u8) -> bool {
        self.buffer.get(self.capacity) == Some(&guard)
    }
}
fn main() {
    let input = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let initial_guard: u8 = 0xAA;
    let mem = MemoryBuffer::new(8, initial_guard);
    let shared_mem = Arc::new(Mutex::new(mem));
    let shared_mem_clone = Arc::clone(&shared_mem);
    let handle = std::thread::spawn(move || {
        let mut buffer = shared_mem_clone.lock().unwrap();
        let res = buffer.write_input(&input);
        if res.is_err() {
        }
    });
    handle.join().unwrap();
    let buffer = shared_mem.lock().unwrap();
    if !buffer.check_guard(initial_guard) {
        panic!("Memory corruption detected: guard value overwritten");
    }
    println!("Execution completed in fixed version");
}
