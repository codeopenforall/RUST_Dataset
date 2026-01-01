use std::thread;
use std::panic;
struct Processor;
impl Processor {
    fn execute(&self) -> Result<(), &'static str> {
        let handle = thread::spawn(|| {
            unsafe {
                let mut data: i32 = 10;
                let ptr: *mut i32 = &mut data;
                *ptr = *ptr - 10;
                if *ptr == 0 {
                    panic!("Division by zero due to invalid operation");
                }
            }
        });
        match handle.join() {
            Ok(_) => Ok(()),
            Err(_) => Err("Recovered from thread panic"),
        }
    }
}
fn run() -> Result<(), &'static str> {
    let proc = Processor;
    proc.execute()
}
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
