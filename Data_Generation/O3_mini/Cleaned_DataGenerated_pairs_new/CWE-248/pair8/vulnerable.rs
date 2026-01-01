use std::thread;
struct Handler;
impl Handler {
    fn new() -> Self {
        Self {}
    }
    fn execute(&self) {
        let th = thread::spawn(|| {
            unsafe {
                let mut value: i32 = 0;
                let raw_ptr: *mut i32 = &mut value;
                if *raw_ptr == 0 {
                    panic!("abnormal termination: unexpected condition encountered");
                }
            }
        });
        th.join().unwrap();
    }
}
fn run_system() {
    let handler = Handler::new();
    handler.execute();
}
fn main() {
    run_system();
}
