use std::thread;
struct Worker;
impl Worker {
    fn process(&self, input: i32) -> Option<i32> {
        let handle = thread::spawn(move || {
            unsafe { Self::unsafe_op(input) }
        });
        Some(handle.join().unwrap())
    }
    unsafe fn unsafe_op(input: i32) -> i32 {
        let raw_ptr = Box::into_raw(Box::new(input));
        if *raw_ptr == 0 {
            panic!("Abnormal termination: input zero");
        }
        let result = *raw_ptr;
        result
    }
}
fn main() {
    let worker = Worker;
    let res = worker.process(0);
    println!("Result: {:?}", res);
}
