use std::env;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
static SHARED: AtomicI32 = AtomicI32::new(0);
const THRESHOLD: i32 = 1000;
pub fn run_app(delta: i32) {
    let mut handles = vec![];
    let threads = 10;
    for _ in 0..threads {
        let d = delta;
        handles.push(thread::spawn(move || {
            for _ in 0..200 {
                loop {
                    let current = SHARED.load(Ordering::SeqCst);
                    if current + d >= THRESHOLD {
                        break;
                    }
                    if SHARED.compare_exchange(current, current + d, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                        break;
                    }
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let final_val = SHARED.load(Ordering::SeqCst);
    assert!(final_val < THRESHOLD, "Safe limit exceeded unexpectedly!");
    println!("Completed run with SHARED value: {}", final_val);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let delta = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    run_app(delta);
}
