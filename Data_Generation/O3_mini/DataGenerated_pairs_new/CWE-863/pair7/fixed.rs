//////////////////////////////////////////////
// Secured concurrent allocation with limits //
//////////////////////////////////////////////
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_TASKS: usize = 100;

struct Worker {
    id: usize,
    data: Vec<u8>,
}

impl Worker {
    // Safe routine that initializes memory using safe constructs.
    fn process(&mut self) {
        // Using safe iteration to initialize the vector.
        for byte in self.data.iter_mut() {
            *byte = 0;
        }
    }
}

fn run_tasks(task_count: usize) {
    // Enforce an upper limit on the number of spawned tasks.
    if task_count > MAX_TASKS {
        panic!("Requested {} tasks, but maximum allowed is {}", task_count, MAX_TASKS);
    }
    
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for i in 0..task_count {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut worker = Worker {
                id: i,
                data: Vec::with_capacity(1024 * 1024),
            };
            // Safely initialize the vector with zeros.
            worker.data.resize(1024 * 1024, 0);
            worker.process();
            let mut guard = shared_clone.lock().unwrap();
            guard.push(worker.id);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn main() {
    // Parameter-driven task allocation with upper bound check.
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    run_tasks(count);
    println!("Completed tasks");
}