use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
static GLOBAL: AtomicU32 = AtomicU32::new(0);
fn run() -> u32 {
    GLOBAL.store(0, Ordering::SeqCst);
    let mut handles = vec![];
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            for _ in 0..1000 {
                GLOBAL.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    GLOBAL.load(Ordering::SeqCst)
}
fn main() {
    let result = run();
    println!("Final counter value: {}", result);
}
