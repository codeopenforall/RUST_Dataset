use std::sync::{Arc, Mutex};
use std::thread;
use std::alloc::{alloc, dealloc, Layout};
struct Processor;
impl Processor {
    pub fn heavy_work(&self, tasks: usize) -> Result<usize, String> {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::with_capacity(tasks);
        for i in 0..tasks {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                unsafe {
                    let size = (i % 1024) + 1; 
                    let layout = Layout::from_size_align(size, 8).unwrap();
                    let mem = alloc(layout);
                    if mem.is_null() {
                        panic!("Memory allocation failed");
                    }
                    *(mem as *mut u8) = (i % 256) as u8;
                    dealloc(mem, layout);
                }
                let mut cnt = counter.lock().unwrap();
                *cnt += 1;
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().map_err(|_| "Thread panicked".to_string())?;
        }
        let result = *counter.lock().unwrap();
        Ok(result)
    }
}
fn main() {
    let processor = Processor;
    let tasks = 1500;
    match processor.heavy_work(tasks) {
        Ok(count) => println!("Completed {} tasks", count),
        Err(e) => println!("Error: {}", e),
    }
}
