use std::thread;
const MAX_ITEMS: usize = 1_000_000;
struct ResourceManager;
impl ResourceManager {
    fn new() -> Self {
        ResourceManager
    }
    fn allocate_items(&self, count: usize) -> Result<Vec<u8>, &'static str> {
         if count > MAX_ITEMS {
             return Err("Requested allocation exceeds limit");
         }
         let vec = vec![0xAA; count];
         Ok(vec)
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
