use std::env;
use std::thread;
const MAX_COUNT: usize = 50;
struct ResourceManager;
impl ResourceManager {
    fn execute(&self, count: usize) -> Result<u32, &'static str> {
        if count > MAX_COUNT {
            return Err("Input exceeds permitted limit");
        }
        let mut buffer = vec![0u32; MAX_COUNT];
        let handler = thread::spawn(move || {
            for i in 0..count {
                buffer[i] = 42;
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
