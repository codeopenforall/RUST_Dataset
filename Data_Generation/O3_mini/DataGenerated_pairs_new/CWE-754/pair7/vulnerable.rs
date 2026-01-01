//////////////////////////////////////////////////////////////////
// Vulnerable Code Example for Exceptional Condition Handling
// This code defines a data fetching module that spawns a thread
// to simulate an asynchronous sensor reading. The main routine
// calls the run() function, which attempts to obtain a value
// from the sensor with a short timeout. However, if a timeout or
// any error occurs, the error branch erroneously returns a default
// numeric result (0) and then increments it unsafely. This
// misinterprets the exceptional condition as a valid value.
//////////////////////////////////////////////////////////////////

use std::sync::mpsc::{channel, RecvTimeoutError};
use std::thread;
use std::time::Duration;

struct DataReader;

impl DataReader {
    // Vulnerable method: returns Ok(value) even on timeout errors.
    pub fn obtain() -> Result<u32, &'static str> {
        // Create a channel to simulate asynchronous data acquisition.
        let (tx, rx) = channel();

        // Spawn a thread to simulate delayed sensor data.
        thread::spawn(move || {
            // Simulate a delay that exceeds the timeout below.
            thread::sleep(Duration::from_millis(50));
            // Send a valid reading.
            let _ = tx.send(42u32);
        });

        // Set a very short timeout so that we expect a timeout.
        let sensor_value = match rx.recv_timeout(Duration::from_millis(10)) {
            Ok(val) => val,
            // Vulnerability: Any error (including timeout) is ignored and
            // mapped to a default value, which is then mistakenly considered valid.
            Err(_err) => 0,
        };

        // Use an unsafe block to perform pointer arithmetic on the value.
        // (Simulating the use of unsafe code in real systems programming.)
        let boxed = Box::new(sensor_value);
        let ptr = Box::into_raw(boxed);
        unsafe {
            // Increment the value pointed to by ptr.
            *ptr = *ptr + 1;
            // Read the updated value.
            let result = *ptr;
            // Manually reclaim the heap allocation.
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