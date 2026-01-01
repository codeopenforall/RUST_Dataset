//////////////////////////////
// Corrected Code Start
//////////////////////////////
use std::env;
use std::thread;

const MAX_TASKS: usize = 100;

struct ResourceManager {
    pool: Vec<thread::JoinHandle<()>>,
}

impl ResourceManager {
    fn new() -> Self {
        ResourceManager { pool: Vec::new() }
    }

    // Now the function enforces an upper limit on the number of tasks spawned.
    fn spawn_tasks(&mut self, n: usize) {
        // Cap the number of tasks to the maximum allowed.
        let limit = if n > MAX_TASKS { MAX_TASKS } else { n };
        for i in 0..limit {
            // Even though this version is also using unsafe for similar semantics,
            // the capacity is properly throttled.
            unsafe {
                let raw_pool = &mut self.pool as *mut Vec<thread::JoinHandle<()>>;
                let handle = thread::spawn(move || {
                    let mut sum = 0;
                    for j in 0..100 {
                        sum += j;
                    }
                    println!("Thread {} computed {}", i, sum);
                });
                (*raw_pool).push(handle);
            }
        }
    }

    fn join_all(&mut self) {
        while let Some(handle) = self.pool.pop() {
            let _ = handle.join();
        }
    }

    // Execute function for testing: spawns tasks and returns the spawned thread count.
    fn execute(&mut self, n: usize) -> usize {
        self.spawn_tasks(n);
        let count = self.pool.len();
        self.join_all();
        count
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: <program> <num>");
        return;
    }
    let num = args[1].parse::<usize>().unwrap_or(0);
    let mut manager = ResourceManager::new();
    manager.spawn_tasks(num);
    manager.join_all();
}
//////////////////////////////
// Corrected Code End
//////////////////////////////