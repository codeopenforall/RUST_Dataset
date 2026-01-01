use std::thread;
pub fn compute(a: i32, b: i32) -> Result<i32, &'static str> {
    unsafe {
        let result = a / b;
        Ok(result)
    }
}
fn main() {
    let res = compute(50, 0);
    match res {
        Ok(val) => println!("Computation result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
    let handle = thread::spawn(|| {
        let res_thread = compute(100, 0);
        match res_thread {
            Ok(val) => println!("Thread result: {}", val),
            Err(e) => println!("Thread error: {}", e),
        }
    });
    let _ = handle.join();
}
