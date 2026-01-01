use std::sync::mpsc;
use std::thread;
struct Module;
trait Operation {
    unsafe fn transform(&self, a: i32, b: i32) -> Result<i32, &'static str>;
}
impl Operation for Module {
    unsafe fn transform(&self, a: i32, b: i32) -> Result<i32, &'static str> {
        if b == 0 {
            return Err("Division by zero error");
        }
        let ptr: *const i32 = &b;
        let factor = *ptr;
        Ok(a / factor)
    }
}
fn compute(a: i32, b: i32) -> Result<i32, &'static str> {
    let m = Module;
    unsafe { m.transform(a, b) }
}
pub fn run_calc(a: i32, b: i32) -> Result<i32, &'static str> {
    compute(a, b)
}
fn main_thread() {
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || {
        let r = compute(10, 0);
        tx.send(r).unwrap();
    });
    let result = rx.recv().unwrap();
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e)
    }
    handle.join().unwrap();
}
fn main() {
    main_thread();
}
