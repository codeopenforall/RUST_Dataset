use std::{thread, panic};
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
    fn run(&self) -> Result<(), &'static str> {
        let result = panic::catch_unwind(|| unsafe {
            trigger(true)
        });
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Recovered from unsafe panic"),
        }
    }
}
pub fn run_app() {
    let handle = thread::spawn(|| {
        let exec = Executor::new();
        if let Err(e) = exec.run() {
            eprintln!("Error: {}", e);
        }
    });
    let _ = handle.join();
}
fn main() {
    run_app();
}
