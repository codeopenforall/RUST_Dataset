use std::thread;
struct ResourceHandler;
impl ResourceHandler {
    pub fn execute_tasks(&self, count: usize) -> Result<(), &'static str> {
        let mut thread_handles = Vec::new();
        for _ in 0..count { 
            let handle = thread::spawn(|| {
                unsafe {
                    let block_size = 1_000_000;
                    let mut buffer = Vec::with_capacity(block_size);
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
    let handler = ResourceHandler;
    let _ = handler.execute_tasks(500);
}
