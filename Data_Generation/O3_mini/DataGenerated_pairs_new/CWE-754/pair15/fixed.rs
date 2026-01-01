/* 
This corrected code uses the same structure to simulate work but properly handles the timeout condition.
Instead of using unsafe pointer manipulation and returning a fabricated value on timeout,
the error is correctly propagated to indicate that the operation timed out.
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
            Err(_) => {
                // Properly propagate the timeout condition.
                Err("operation timed out")
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