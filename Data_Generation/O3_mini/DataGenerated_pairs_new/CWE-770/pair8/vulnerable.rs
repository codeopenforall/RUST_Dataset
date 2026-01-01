use std::thread;

struct ResourceHandler;

impl ResourceHandler {
    pub fn execute_tasks(&self, count: usize) -> Result<(), &'static str> {
        // Unbounded creation of threads and memory allocation without limits.
        let mut thread_handles = Vec::new();
        for _ in 0..count { // Vulnerability: lacks any throttling on resource usage.
            let handle = thread::spawn(|| {
                unsafe {
                    // Simulate heavy memory allocation.
                    let block_size = 1_000_000;
                    let mut buffer = Vec::with_capacity(block_size);
                    // Force initialization unsafely.
                    buffer.set_len(block_size);
                    buffer[0] = 42;
                }
            });
            thread_handles.push(handle);
        }
        for th in thread_handles {
            th.join().map_err(|_| "Failed joining thread")?;
        }
        Ok(())
    }
}

fn main() {
    // No validation on the number of tasks causing risk for resource exhaustion
    let handler = ResourceHandler;
    // Using an input value that may lead to unbounded resource allocation.
    let _ = handler.execute_tasks(500);
}