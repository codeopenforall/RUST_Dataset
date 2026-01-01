use std::sync::mpsc;
use std::thread;
use std::time::Duration;
struct Application;
impl Application {
    fn execute(&self) -> Result<u32, &'static str> {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            unsafe {
                let data = 42u32;
                let raw_ptr = &data as *const u32;
                thread::sleep(Duration::from_millis(150));
                let _ = tx.send(*raw_ptr);
            }
        });
        let result = rx.recv_timeout(Duration::from_millis(100));
        match result {
            Ok(num) => Ok(num),
            Err(_) => {
                Ok(0)
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
