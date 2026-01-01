/* 
This Rust application unsafely updates a global static variable concurrently.
It uses an unsafe block to perform unsynchronized arithmetic on a mutable static variable.
An attacker can supply a large delta as a command‐line argument so that the final value exceeds the defined threshold.
When that happens, a reachable assert! (line 27) is triggered, leading to process abort (DoS).
*/
use std::env;
use std::thread;

static mut SHARED: i32 = 0;
const THRESHOLD: i32 = 1000;

pub fn run_app(delta: i32) {
    // Spawn threads that update the shared state unsafely.
    let mut handles = vec![];
    let threads = 10;
    for _ in 0..threads {
        let d = delta;
        handles.push(thread::spawn(move || {
            for _ in 0..200 {
                unsafe {
                    // Vulnerability: unsynchronized update leads to a data race.
                    SHARED = SHARED.wrapping_add(d);
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    unsafe {
        // Vulnerability: reachable assertion on attacker-influenced state.
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