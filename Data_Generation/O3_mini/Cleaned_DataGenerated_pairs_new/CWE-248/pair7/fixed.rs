use std::thread;
use std::panic;
struct Worker;
impl Worker {
    fn process(&self, input: i32) -> Option<i32> {
        let handle = thread::spawn(move || {
            panic::catch_unwind(|| unsafe { Self::unsafe_op(input) })
                .unwrap_or(-1)
        });
        Some(handle.join().unwrap())
    }
    unsafe fn unsafe_op(input: i32) -> i32 {
        let boxed = Box::new(input);
        let raw_ptr = Box::into_raw(boxed);
        let result;
        if *raw_ptr == 0 {
            result = -1;
        } else {
            result = *raw_ptr;
        }
        drop(Box::from_raw(raw_ptr));
        result
    }
}
fn main() {
    let worker = Worker;
    let res = worker.process(0);
    match res {
        Some(val) if val == -1 => println!("Handled error gracefully"),
        Some(val) => println!("Result: {}", val),
        None => println!("No result obtained"),
    }
}
