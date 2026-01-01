use std::env;
use std::thread;
use std::sync::{Arc, Mutex};
static mut GLOBAL_RESOURCE: Option<Arc<Mutex<Vec<u8>>>> = None;
fn spawn_workers(count: usize) {
    let resource = Arc::new(Mutex::new(Vec::new()));
    unsafe {
        GLOBAL_RESOURCE = Some(resource.clone());
    }
    for _ in 0..count {
        let res = resource.clone();
        thread::spawn(move || {
            unsafe {
                let mut guard = res.lock().unwrap();
                let ptr = guard.as_mut_ptr();
                if !ptr.is_null() {
                    *ptr = 42;
                }
            }
        });
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let count = args.get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1_000_000);
    spawn_workers(count);
    println!("Spawned {} workers", count);
}
