use std::env;
use std::sync::Arc;
use std::thread;
trait Processor {
    fn process(&self, count: usize) -> Result<u64, String>;
}
struct Manager;
impl Manager {
    unsafe fn unsafe_allocate(&self, size: usize) -> Vec<u8> {
        let mut vec = Vec::with_capacity(size);
        vec.set_len(size);
        for elem in vec.iter_mut() {
            *elem = 1; 
        }
        vec
    }
}
impl Processor for Manager {
    fn process(&self, count: usize) -> Result<u64, String> {
        let mut handles = Vec::new();
        let mut total: u64 = 0;
        let data = unsafe { self.unsafe_allocate(count) };
        let chunk_size = if count < 4 { 1 } else { count / 4 };
        let num_threads = if count < 4 { 1 } else { 4 };
        let shared_data = Arc::new(data);
        for idx in 0..num_threads {
            let cloned_data = Arc::clone(&shared_data);
            let handle = thread::spawn(move || -> u64 {
                let start = idx * chunk_size;
                let end = if idx == num_threads - 1 {
                    cloned_data.len()
                } else {
                    start + chunk_size
                };
                let mut local_sum = 0u64;
                for i in start..end {
                    local_sum += cloned_data[i] as u64;
                }
                local_sum
            });
            handles.push(handle);
        }
        for handle in handles {
            match handle.join() {
                Ok(val) => total += val,
                Err(_) => return Err("A thread panicked".to_string()),
            }
        }
        Ok(total)
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };
    let instance = Manager;
    match instance.process(count) {
        Ok(result) => println!("Final Result: {}", result),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}
