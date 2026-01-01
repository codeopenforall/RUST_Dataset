use std::thread;
struct Processor;
impl Processor {
    fn execute(&self) {
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
        let _ = handle.join().unwrap();
    }
}
fn run() {
    let proc = Processor;
    proc.execute();
}
fn main() {
    run();
}
