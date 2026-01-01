use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Application;

impl Application {
    fn execute(&self) -> Result<u32, &'static str> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            unsafe {
                // simulate unsafe access: obtain a raw pointer to a local variable
                let data = 42u32;
                let raw_ptr = &data as *const u32;
                thread::sleep(Duration::from_millis(150));
                // unsafely dereference the pointer and send its value
                let value = *raw_ptr;
                let _ = tx.send(value);
            }
        });
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(num) => Ok(num),
            Err(_) => {
                // Correct behavior: propagate the exceptional condition as an error.
                Err("Operation timed out")
            }
        }
    }
}

fn main() {
    let app = Application;
    match app.execute() {
        Ok(value) => println!("Operation succeeded with value: {}", value),
        Err(err) => eprintln!("Operation failed: {}", err),
    }
}