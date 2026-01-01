//////////////////////
// VULNERABLE CODE
//////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct Server {
    tasks: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl Server {
    fn new() -> Self {
        Server {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // This method spawns one thread per task without imposing any upper bound.
    // It uses an unsafe block to perform low-level pointer arithmetic to populate a buffer.
    // There is no check on 'count', so an excessively high input may exhaust system resources.
    fn run(&self, count: usize) -> Result<(), &'static str> {
        let mut handles = Vec::with_capacity(count);
        for _ in 0..count {
            let tasks = Arc::clone(&self.tasks);
            handles.push(thread::spawn(move || {
                unsafe {
                    // Allocate a buffer with capacity for 1024 bytes.
                    let mut buf: Vec<u8> = Vec::with_capacity(1024);
                    let ptr = buf.as_mut_ptr();
                    // Using raw pointer arithmetic to initialize the buffer.
                    for j in 0..1024 {
                        *ptr.add(j) = 0;
                    }
                    // Inform the vector that its length is now 1024.
                    buf.set_len(1024);
                    // Push the buffer into the shared vector without any limit.
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
    // Read task count from command-line argument (default: 100)
    let arg = std::env::args().nth(1).unwrap_or_else(|| "100".to_string());
    let count = arg.parse::<usize>().unwrap();
    let srv = Server::new();
    match srv.run(count) {
        Ok(_) => println!("Handled {} tasks", count),
        Err(e) => println!("Error: {}", e),
    }
}