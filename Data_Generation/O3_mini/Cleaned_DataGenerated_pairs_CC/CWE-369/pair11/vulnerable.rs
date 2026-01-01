use std::sync::mpsc;
use std::thread;
use std::panic;
struct Module;
trait Operation {
    unsafe fn transform(&self, a: i32, b: i32) -> i32;
}
impl Operation for Module {
    unsafe fn transform(&self, a: i32, b: i32) -> i32 {
        let ptr: *const i32 = &b;
        let factor = *ptr;
        a / factor
    }
}
fn compute(a: i32, b: i32) -> i32 {
    let m = Module;
    unsafe { m.transform(a, b) }
}
pub fn run_calc(a: i32, b: i32) -> Result<i32, &'static str> {
    let res = panic::catch_unwind(|| compute(a, b));
    match res {
        Ok(val) => Ok(val),
        Err(_) => Err("Division by zero occurred"),
    }
}
fn main_thread() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let r = compute(10, 0);
        tx.send(r).unwrap();
    });
    let _ = rx.recv().unwrap();
    handle.join().unwrap();
}
fn main() {
    main_thread();
}
