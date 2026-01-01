use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::Duration;
#[derive(Debug, PartialEq)]
enum ExecError {
    Timeout,
    Cancel,
    Internal,
}
struct Manager;
impl Manager {
    fn execute(&self, duration: Duration) -> Result<u32, ExecError> {
        let (tx, rx) = channel();
        let shared: Arc<Mutex<u32>> = Arc::new(Mutex::new(0));
        let shared_clone = Arc::clone(&shared);
        thread::spawn(move || {
            if duration < Duration::from_millis(50) {
                unsafe {
                    let ptr: *mut u32 = &mut *shared_clone.lock().unwrap();
                    *ptr = 0;
                }
                let _ = tx.send(Err(ExecError::Timeout));
            } else {
                unsafe {
                    let ptr: *mut u32 = &mut *shared_clone.lock().unwrap();
                    *ptr = 42;
                }
                let _ = tx.send(Ok(42));
            }
        });
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(val)) => Ok(val),
            Ok(Err(e)) => Err(e),
            Err(RecvTimeoutError::Timeout) => Err(ExecError::Timeout),
            Err(_) => Err(ExecError::Internal),
        }
    }
}
fn main() {
    let manager = Manager;
    match manager.execute(Duration::from_millis(30)) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {:?}", e),
    }
}
