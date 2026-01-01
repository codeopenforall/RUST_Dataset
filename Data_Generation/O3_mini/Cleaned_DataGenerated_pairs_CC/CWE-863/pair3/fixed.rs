use std::thread;
const MAX_THREADS: usize = 1000;
const MAX_ALLOCATION: usize = 10 * 1024 * 1024; 
struct ResourceManager;
impl ResourceManager {
    pub fn process_request(count: usize, size: usize) -> Result<u8, &'static str> {
        if count > MAX_THREADS || size > MAX_ALLOCATION {
            return Err("Parameters exceed safe limits");
        }
        let mut handles = Vec::new();
        let mut total = 0u8;
        for _ in 0..count {
            handles.push(thread::spawn(move || {
                unsafe {
                    let mut buffer: Vec<u8> = Vec::with_capacity(size);
                    buffer.set_len(size);
                    for i in 0..size {
                        *buffer.get_unchecked_mut(i) = 42;
                    }
                    buffer.iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
                }
            }));
        }
        for handle in handles {
            total = total.wrapping_add(handle.join().unwrap());
        }
        Ok(total)
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let count: usize = args.get(1).unwrap_or(&"1000".to_string()).parse().unwrap();
    let size: usize = args.get(2).unwrap_or(&"1048576".to_string()).parse().unwrap();
    match ResourceManager::process_request(count, size) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
