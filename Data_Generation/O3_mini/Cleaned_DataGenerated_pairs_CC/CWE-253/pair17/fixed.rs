use std::sync::{Arc, Mutex};
use std::thread;
struct Worker {
    data: Box<[u32]>,
}
impl Worker {
    fn new(size: usize) -> Result<Self, &'static str> {
        if size == 0 {
            Err("Invalid size for Worker initialization")
        } else {
            Ok(Worker { data: vec![1; size].into_boxed_slice() })
        }
    }
    fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        if idx >= self.data.len() {
            Err("Index out of bounds")
        } else {
            unsafe {
                let ptr = self.data.as_mut_ptr().add(idx);
                *ptr = val;
            }
            Ok(())
        }
    }
}
fn process_input(input: i32) -> i32 {
    let size = if input < 0 { 0 } else { 10 };
    let worker_instance = match Worker::new(size) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error during worker initialization: {}", e);
            return -1; 
        }
    };
    let worker = Arc::new(Mutex::new(worker_instance));
    let mut handles = Vec::new();
    for i in 0..5 {
        let worker_arc = Arc::clone(&worker);
        handles.push(thread::spawn(move || {
            let mut guard = worker_arc.lock().unwrap();
            if let Err(e) = guard.update((i * 2) as usize, (i * 10) as u32) {
                eprintln!("Update error in thread {}: {}", i, e);
                return;
            }
        }));
    }
    for handle in handles {
        if let Err(_) = handle.join() {
            eprintln!("A thread panicked during execution.");
            return -1;
        }
    }
    let guard = worker.lock().unwrap();
    guard.data.iter().sum::<u32>() as i32
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    let result = process_input(input);
    if result == -1 {
        eprintln!("Processing failed due to invalid input or runtime error.");
        std::process::exit(1);
    } else {
        println!("Result: {}", result);
    }
}
