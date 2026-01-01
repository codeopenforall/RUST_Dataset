use std::env;
use std::thread;
struct ResourceManager;
impl ResourceManager {
    fn execute(&self, count: usize) -> Result<u32, &'static str> {
        let mut buffer = vec![0u32; 50];
        let handler = thread::spawn(move || {
            for i in 0..count {
                unsafe {
                    let ptr = buffer.as_mut_ptr();
                    *ptr.add(i) = 42;
                }
            }
            buffer.iter().sum()
        });
        handler.join().map_err(|_| "Thread execution error")
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <count>", args[0]);
        std::process::exit(1);
    }
    let count: usize = args[1].parse().unwrap_or(0);
    let manager = ResourceManager;
    match manager.execute(count) {
        Ok(sum) => println!("Operation result: {}", sum),
        Err(e) => eprintln!("Error occurred: {}", e),
    }
}
