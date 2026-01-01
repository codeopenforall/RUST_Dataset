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

    // Safely writes input to the buffer after verifying bounds.
    // Returns an error if input exceeds the intended capacity.
    fn write_input(&mut self, input: &[u8]) -> Result<(), &'static str> {
        if input.len() > self.capacity {
            // Reject writes that would corrupt the guard area.
            return Err("Input length exceeds buffer capacity");
        }
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            for i in 0..input.len() {
                *ptr.add(i) = input[i];
            }
            // Adjust the length only within safe bounds.
            if input.len() > self.buffer.len() {
                self.buffer.set_len(input.len());
            }
        }
        Ok(())
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
        // In the fixed version, attempt to write input beyond capacity returns an error.
        let res = buffer.write_input(&input);
        if res.is_err() {
            // Proper handling in main thread; in a real-world scenario, one might log or handle the error.
            // Here we simply ignore the faulty write.
        }
    });
    handle.join().unwrap();

    let buffer = shared_mem.lock().unwrap();
    // Ensure the guard value remains intact.
    if !buffer.check_guard(initial_guard) {
        panic!("Memory corruption detected: guard value overwritten");
    }
    println!("Execution completed in fixed version");
}