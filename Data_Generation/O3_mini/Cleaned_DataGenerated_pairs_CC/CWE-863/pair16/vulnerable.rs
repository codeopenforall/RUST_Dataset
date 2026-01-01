use std::thread;
use std::ptr;
struct ResourceManager;
impl ResourceManager {
    fn new() -> Self {
        ResourceManager
    }
    fn allocate_items(&self, count: usize) -> Result<Vec<u8>, &'static str> {
        unsafe {
            let mut vec = Vec::with_capacity(count);
            vec.set_len(count);
            for i in 0..count {
                *vec.get_unchecked_mut(i) = 0xAA;
            }
            Ok(vec)
        }
    }
}
fn process(count: usize) -> Result<Vec<u8>, &'static str> {
    let manager = ResourceManager::new();
    let handle = thread::spawn(move || {
         manager.allocate_items(count)
    });
    handle.join().unwrap()
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let count = if args.len() > 1 {
         args[1].parse().unwrap_or(0)
    } else {
         0
    };
    match process(count) {
         Ok(vec) => println!("Allocation completed, size: {}", vec.len()),
         Err(e) => println!("Failed: {}", e),
    }
}
