use std::sync::Arc;
use std::thread;

struct Data {
    ptr: *const u8,
    length: usize,
}

impl Data {
    // Safety: The caller must guarantee that "ptr" is valid for "length" bytes.
    unsafe fn as_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.ptr, self.length)
    }
}

fn compute_sum(shared: Arc<Data>) -> u32 {
    let mut handles = Vec::new();

    // Spawn a few threads to process the shared data concurrently.
    for _ in 0..4 {
        let clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                // This unsafe block does not revalidate the design-time precondition:
                // "length" must match the allocated buffer.
                let slice = clone.as_slice();
                // Sum each byte widening it to u32.
                slice.iter().map(|&b| b as u32).sum::<u32>()
            }
        });
        handles.push(handle);
    }

    let mut total = 0;
    for handle in handles {
        // join may yield undefined behavior if the unsafe block accessed invalid memory.
        total += handle.join().unwrap();
    }
    total
}

fn main() {
    // Allocated vector with exactly 4 bytes.
    let mut valid_data = vec![1u8, 2, 3, 4];
    // **** Vulnerability Trigger ****
    // The length provided exceeds the actual allocation, violating the API precondition.
    let instance = Data {
        ptr: valid_data.as_ptr(),
        length: valid_data.len() + 10, // Undefined behavior: reading beyond allocated memory.
    };

    let shared_instance = Arc::new(instance);
    let result = compute_sum(shared_instance);
    println!("Result: {}", result);
}