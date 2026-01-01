use std::sync::{Arc, Mutex};
use std::thread;
const MAX_LIMIT: usize = 1000;
struct Server {
    tasks: Arc<Mutex<Vec<Vec<u8>>>>,
}
impl Server {
    fn new() -> Self {
        Server {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn run(&self, count: usize) -> Result<(), &'static str> {
        if count > MAX_LIMIT {
            return Err("Count exceeds maximum allowed limit");
        }
        let mut handles = Vec::with_capacity(count);
        for _ in 0..count {
            let tasks = Arc::clone(&self.tasks);
            handles.push(thread::spawn(move || {
                unsafe {
                    let mut buf: Vec<u8> = Vec::with_capacity(1024);
                    let ptr = buf.as_mut_ptr();
                    for j in 0..1024 {
                        *ptr.add(j) = 0;
                    }
                    buf.set_len(1024);
                    let mut lock = tasks.lock().unwrap();
                    lock.push(buf);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        Ok(())
    }
}
fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "100".to_string());
    let count = arg.parse::<usize>().unwrap();
    let srv = Server::new();
    match srv.run(count) {
        Ok(_) => println!("Handled {} tasks", count),
        Err(e) => println!("Error: {}", e),
    }
}
