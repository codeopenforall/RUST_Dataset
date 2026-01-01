///////////////////////////////////////////////////////////////
// Fixed Code Sample
///////////////////////////////////////////////////////////////
use std::thread;
use std::sync::{Arc, Mutex};

const MAX_THREADS: usize = 100;      // Cap on the number of spawned tasks.
const MAX_ALLOCATION: usize = 10_000;  // Cap on the allocation size per task.

pub struct Manager {
    data: Arc<Mutex<Vec<u8>>>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Safe version: limits the number of threads and the allocation size.
    pub fn spawn_tasks(&self, count: usize, allocation: usize) {
        // Enforce resource limits.
        let safe_count = if count > MAX_THREADS { MAX_THREADS } else { count };
        let safe_alloc = if allocation > MAX_ALLOCATION {
            MAX_ALLOCATION
        } else {
            allocation
        };

        let mut handles = Vec::new();
        for _ in 0..safe_count {
            let shared = self.data.clone();
            let handle = thread::spawn(move || {
                // Instead of using unsafe set_len, we safely create a vector filled with zeros.
                let buf = vec![0u8; safe_alloc];
                // Compute a defined result.
                let calc = buf.iter().fold(0, |acc, &val| acc + val as usize);
                let mut locked = shared.lock().unwrap();
                locked.push((calc % 256) as u8);
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    pub fn process(&self) -> u8 {
        let locked = self.data.lock().unwrap();
        locked.iter().fold(0, |acc, &x| acc.wrapping_add(x))
    }
}

pub fn execute(count: usize, allocation: usize) -> u8 {
    let mgr = Manager::new();
    mgr.spawn_tasks(count, allocation);
    mgr.process()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cnt = if args.len() > 1 {
        args[1].parse().unwrap_or(100)
    } else {
        100
    };
    let alloc = if args.len() > 2 {
        args[2].parse().unwrap_or(1000)
    } else {
        1000
    };
    let res = execute(cnt, alloc);
    println!("Result: {}", res);
}