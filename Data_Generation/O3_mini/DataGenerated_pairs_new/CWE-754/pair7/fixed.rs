//////////////////////////////////////////////////////////////////
// Fixed Code Example for Exceptional Condition Handling
// This code implements the same asynchronous data retrieval logic,
// but it properly distinguishes between a valid reading and a timeout.
// If a timeout occurs, the obtain() function now returns an error, preventing
// the unsafe manipulation of an invalid default value.
//////////////////////////////////////////////////////////////////

use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::Duration;

struct DataReader;

impl DataReader {
    // Corrected method: returns an error on timeout or other exceptional states.
    pub fn obtain() -> Result<u32, &'static str> {
        // Create a channel for asynchronous sensor data.
        let (tx, rx) = channel();

        // Spawn a thread to simulate delayed sensor acquisition.
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let _ = tx.send(42u32);
        });

        // Wait for the sensor reading with a timeout.
        let sensor_value = match rx.recv_timeout(Duration::from_millis(10)) {
            Ok(val) => val,
            Err(RecvTimeoutError::Timeout) => return Err("timeout"),
            Err(RecvTimeoutError::Disconnected) => return Err("disconnected"),
        };

        // Safely increment the sensor value using an unsafe block.
        // Since sensor_value is proven valid, we can safely perform pointer math.
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