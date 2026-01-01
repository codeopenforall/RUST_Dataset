use std::env;
use std::thread;
use std::sync::{Arc, Mutex};
static mut GLOBAL_RESOURCE: Option<Arc<Mutex<Vec<u8>>>> = None;
const MAX_WORKERS: usize = 100; 
fn spawn_workers(count: usize) {
    let count = if count > MAX_WORKERS { MAX_WORKERS } else { count };
    let resource = Arc::new(Mutex::new(vec![0u8; 1]));
    unsafe {
        GLOBAL_RESOURCE = Some(resource.clone());
    }
    let mut handles = Vec::with_capacity(count);
    for _ in 0..count {
        let res = resource.clone();
        handles.push(thread::spawn(move || {
            let mut guard = res.lock().unwrap();
            guard[0] = 42; 
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let count = args.get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(50);
    spawn_workers(count);
    println!("Spawned {} workers (capped)", count);
}
