use std::sync::mpsc;
use std::thread;
use std::time::Duration;
pub fn process(input: u32) -> Result<u32, &'static str> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        if input == 1 {
            thread::sleep(Duration::from_millis(100));
        }
        unsafe {
            let ptr = &input as *const u32;
            let value = *ptr;
            let _ = tx.send(value);
        }
    });
    match rx.recv_timeout(Duration::from_millis(50)) {
        Ok(v) => Ok(v),
        Err(e) => {
            if let mpsc::RecvTimeoutError::Timeout = e {
                Ok(0)
            } else {
                Err("Channel disconnected")
            }
        }
    }
}
fn main() {
    match process(2) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}
