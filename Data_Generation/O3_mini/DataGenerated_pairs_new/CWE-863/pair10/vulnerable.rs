//////////////////////////////
// Vulnerable Code Start
//////////////////////////////
use std::env;
use std::thread;

struct ResourceManager {
    pool: Vec<thread::JoinHandle<()>>,
}

impl ResourceManager {
    fn new() -> Self {
        ResourceManager { pool: Vec::new() }
    }

    // This function spawns tasks without any proper cap, and it uses an unsafe block
    // to push new thread handles into the pool.
    fn spawn_tasks(&mut self, n: usize) {
        for i in 0..n {
            unsafe {
                // Create a raw mutable pointer to the tasks pool.
                let raw_pool = &mut self.pool as *mut Vec<thread::JoinHandle<()>>;
                // Spawn a thread doing simple computation.
                let handle = thread::spawn(move || {
                    let mut sum = 0;
                    for j in 0..100 {
                        sum += j;
                    }
                    // Print the thread id and computed sum.
                    println!("Thread {} computed {}", i, sum);
                });
                // Unsafely push the thread handle into the vector.
                (*raw_pool).push(handle);
            }
        }
    }

    // join all threads: empties the pool.
    fn join_all(&mut self) {
        while let Some(handle) = self.pool.pop() {
            let _ = handle.join();
        }
    }

    // Execute function for testing: spawns n tasks and returns the number of threads spawned.
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
// Vulnerable Code End
//////////////////////////////