use std::{thread};
unsafe fn trigger(flag: bool) {
    if flag {
        panic!("Abnormal termination encountered!");
    }
}
struct Executor;
impl Executor {
    fn new() -> Self {
        Self
    }
    fn run(&self) {
        unsafe {
            trigger(true)
        }
    }
}
pub fn run_app() {
    let handle = thread::spawn(|| {
        let exec = Executor::new();
        exec.run();
    });
    let _ = handle.join().unwrap();
}
fn main() {
    run_app();
}
