use std::thread;
unsafe fn compute(val: i32) -> i32 {
    if val < 0 {
        panic!("Invalid value: negative input not allowed");
    }
    val * 2
}
fn run_task(input: i32) -> Result<i32, String> {
    let handle = thread::spawn(move || {
        unsafe { compute(input) }
    });
    Ok(handle.join().unwrap())
}
fn main() {
    let _ = run_task(-1);
}
