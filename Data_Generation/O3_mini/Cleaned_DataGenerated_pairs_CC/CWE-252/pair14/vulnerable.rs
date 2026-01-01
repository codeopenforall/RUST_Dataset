use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
struct Executor {
    flag: Arc<AtomicBool>,
}
impl Executor {
    fn new() -> Self {
        Executor { flag: Arc::new(AtomicBool::new(false)) }
    }
    fn run(&self, input: u32) -> bool {
        let flag = self.flag.clone();
        let handle = thread::spawn(move || {
            unsafe {
                let _ = Self::unsafe_op(input);
            }
            flag.store(true, Ordering::Relaxed);
        });
        handle.join().unwrap();
        self.flag.load(Ordering::Relaxed)
    }
    unsafe fn unsafe_op(input: u32) -> Result<(), &'static str> {
        if input == 42 {
            Err("Operation encountered an error")
        } else {
            Ok(())
        }
    }
}
fn process_input(input: u32) -> bool {
    let exec = Executor::new();
    exec.run(input)
}
fn main() {
    let input = 42;
    let outcome = process_input(input);
    println!("Outcome: {}", outcome);
}
