use std::thread;
use std::sync::{Arc, Mutex};
pub struct Manager {
    data: Arc<Mutex<Vec<u8>>>,
}
impl Manager {
    pub fn new() -> Manager {
        Manager {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn spawn_tasks(&self, count: usize, allocation: usize) {
        let mut handles = Vec::new();
        for _ in 0..count {
            let shared = self.data.clone();
            let handle = thread::spawn(move || {
                unsafe {
                    let mut buf: Vec<u8> = Vec::with_capacity(allocation);
                    buf.set_len(allocation); 
                    let calc = buf.iter().fold(0, |acc, &val| acc + val as usize);
                    let mut locked = shared.lock().unwrap();
                    locked.push((calc % 256) as u8);
                }
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
