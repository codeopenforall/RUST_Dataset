use std::env;
use std::thread;
static mut GLOBAL: u64 = 0;
const MAX_THREADS: u64 = 1000; 
pub fn compute(n: u64) -> u64 {
    unsafe {
        GLOBAL = 0;
    }
    let count = if n > MAX_THREADS { MAX_THREADS } else { n };
    let mut handles = vec![];
    for _ in 0..count {
        handles.push(thread::spawn(|| {
            unsafe {
                let val = GLOBAL;
                let _dummy = (0..1000).fold(0, |acc, x| acc + x);
                GLOBAL = val + 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    unsafe { GLOBAL }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args.get(1)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    let result = compute(n);
    println!("Result: {}", result);
}
