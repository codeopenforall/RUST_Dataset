use std::env;
use std::thread;
static mut SHARED: i32 = 0;
const THRESHOLD: i32 = 1000;
pub fn run_app(delta: i32) {
    let mut handles = vec![];
    let threads = 10;
    for _ in 0..threads {
        let d = delta;
        handles.push(thread::spawn(move || {
            for _ in 0..200 {
                unsafe {
                    SHARED = SHARED.wrapping_add(d);
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    unsafe {
        assert!(SHARED < THRESHOLD, "Global variable exceeded safe limit!");
        println!("Completed run with SHARED value: {}", SHARED);
    }
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
