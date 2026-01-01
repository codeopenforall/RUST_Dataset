use std::thread;
trait Operation {
    fn process(&self) -> i32;
}
struct Handler;
impl Operation for Handler {
    fn process(&self) -> i32 {
        let boxed = Box::new(42);
        let raw_ptr = Box::into_raw(boxed);
        unsafe {
            drop(Box::from_raw(raw_ptr));  
            let handle = thread::spawn(|| {
                let _dummy = vec![0u8; 1024];
            });
            handle.join().unwrap();
            (*raw_ptr)  
        }
    }
}
pub fn compute() -> i32 {
    let proc = Handler;
    proc.process()
}
fn main() {
    let result = compute();
    println!("Result: {}", result);
}
