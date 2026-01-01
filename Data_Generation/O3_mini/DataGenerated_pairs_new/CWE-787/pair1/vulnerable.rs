use std::sync::{Arc, Mutex};

struct MemoryBuffer {
    buffer: Vec<u8>,
    capacity: usize,
}

impl MemoryBuffer {
    // Allocates a vector with capacity + 1 elements.
    // The extra element is used as a guard value.
    fn new(capacity: usize, guard: u8) -> Self {
        let mut vec = Vec::with_capacity(capacity + 1);
        // Initialize the vector to have capacity+1 elements.
        unsafe {
            vec.set_len(capacity + 1);
        }
        // Set the guard value at the extra slot.
        vec[capacity] = guard;
        Self {
            buffer: vec,
            capacity,
        }
    }

    // Writes the input bytes into the buffer without boundary check.
    // If input length exceeds the intended capacity, the guard gets overwritten.
    fn write_input(&mut self, input: &[u8]) {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            // Vulnerability: unchecked write in loop; out-of-bounds writes occur if input.len() > capacity.
            for i in 0..input.len() {
                *ptr.add(i) = input[i];
            }
            // In some cases, set_len is misused to reflect new buffer length.
            if input.len() > self.buffer.len() {
                self.buffer.set_len(input.len());
            }
        }
    }

    // Returns true if guard value is intact.
    fn check_guard(&self, guard: u8) -> bool {
        self.buffer.get(self.capacity) == Some(&guard)
    }
}

fn main() {
    // Input deliberately longer than the capacity defined.
    let input = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let initial_guard: u8 = 0xAA;
    let mem = MemoryBuffer::new(8, initial_guard);
    
    // Wrap the buffer in Arc Mutex to mimic concurrency.
    let shared_mem = Arc::new(Mutex::new(mem));
    let shared_mem_clone = Arc::clone(&shared_mem);
    
    let handle = std::thread::spawn(move || {
        let mut buffer = shared_mem_clone.lock().unwrap();
        buffer.write_input(&input);
    });
    handle.join().unwrap();

    let buffer = shared_mem.lock().unwrap();
    // If the guard value has been corrupted due to out-of-bounds writes, panic.
    if !buffer.check_guard(initial_guard) {
        panic!("Memory corruption detected: guard value overwritten");
    }
    println!("Execution completed in vulnerable version");
}