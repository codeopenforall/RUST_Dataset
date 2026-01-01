use std::env;
use std::thread;
struct ResourceManager {
    pool: Vec<thread::JoinHandle<()>>,
}
impl ResourceManager {
    fn new() -> Self {
        ResourceManager { pool: Vec::new() }
    }
    fn spawn_tasks(&mut self, n: usize) {
        for i in 0..n {
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
