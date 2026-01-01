use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::Duration;
struct DataReader;
impl DataReader {
    pub fn obtain() -> Result<u32, &'static str> {
        let (tx, rx) = channel();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let _ = tx.send(42u32);
        });
        let sensor_value = match rx.recv_timeout(Duration::from_millis(10)) {
            Ok(val) => val,
            Err(RecvTimeoutError::Timeout) => return Err("timeout"),
            Err(RecvTimeoutError::Disconnected) => return Err("disconnected"),
        };
        let boxed = Box::new(sensor_value);
        let ptr = Box::into_raw(boxed);
        unsafe {
            *ptr = *ptr + 1;
            let result = *ptr;
            Box::from_raw(ptr);
            Ok(result)
        }
    }
}
fn main() {
    match DataReader::obtain() {
        Ok(val) => println!("Obtained reading: {}", val),
        Err(e) => println!("Error obtaining reading: {}", e),
    }
}
