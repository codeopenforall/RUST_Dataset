use std::thread;
const MAX_MEMORY_ALLOCATION: usize = 100_000; 
const MAX_THREAD_COUNT: usize = 500; 
struct ResourceManager;
impl ResourceManager {
    pub fn create_resources(&self, count: usize) -> Result<Vec<u8>, String> {
        if count > MAX_MEMORY_ALLOCATION {
            return Err("Requested resources exceed safe limit".to_string());
        }
        unsafe {
            let mut buf: Vec<u8> = Vec::with_capacity(count);
            for i in 0..count {
                buf.push((i % 256) as u8);
            }
            Ok(buf)
        }
    }
    pub fn spawn_workers(&self, count: usize) {
        let safe_count = if count > MAX_THREAD_COUNT { MAX_THREAD_COUNT } else { count };
        let mut handles = Vec::new();
        for i in 0..safe_count {
            let handle = thread::spawn(move || {
                unsafe {
                    let ptr: *const usize = &i;
                    let _ = *ptr;
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
    }
}
fn main() {
    let manager = ResourceManager;
    let resources = manager.create_resources(50_000).expect("Allocation within limit");
    println!("Safely allocated {} bytes", resources.len());
    manager.spawn_workers(600); 
}
