////////////////////////////////////////////////////////
// Test Oracle Function
////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_run() {
        // Create an instance of the Data structure.
        let inst = Data::new();
        // Use a channel to control test execution time.
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            // Run the operation that should return a value.
            let res = inst.run();
            tx.send(res).unwrap();
        });

        // Wait for at most 500 milliseconds for a result.
        match rx.recv_timeout(Duration::from_millis(500)) {
            Ok(result) => assert_eq!(result, 3, "Expected the sum to be 3"),
            Err(_) => panic!("Operation timed out: potential deadlock detected"),
        }
    }
}