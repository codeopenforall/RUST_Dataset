/* 
This code simulates an engine that spawns a worker thread to compute a value.
A timeout is set deliberately shorter than the worker’s sleep duration to trigger a timeout.
Due to an improper check for exceptional conditions, instead of propagating the timeout error,
an unsafe block is executed to return a default value. This unsound handling may cause further issues.
*/
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Engine;

impl Engine {
    fn run(&self) -> Result<i32, &'static str> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            // Simulate work taking longer than the timeout threshold.
            thread::sleep(Duration::from_millis(100));
            let computed = 123;
            let _ = tx.send(computed);
        });
        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(val) => Ok(val),
            Err(_) => { // Improper handling of exceptional condition: timeout is misinterpreted.
                unsafe {
                    // Unsafe block: using raw pointer manipulation to manufacture a default value.
                    let mut dummy: i32 = 0;
                    let ptr = &mut dummy as *mut i32;
                    // The intent was to signal an error, but instead a default value is written.
                    *ptr = 42;
                    Ok(*ptr)
                }
            }
        }
    }
}

fn main() {
    let eng = Engine;
    match eng.run() {
        Ok(result) => println!("Computed value: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}