/////////////////////////////
// Test Oracle Start Here  //
/////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::time::Duration;
    use std::thread;
    // Import the run function from the binary crate.
    use super::run;

    #[test]
    fn test_run() {
        // Spawn the run() function in a separate thread and wait for completion.
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let res = run();
            tx.send(res).expect("Failed to send result");
        });

        // If the vulnerable code deadlocks, this will timeout and the test will panic.
        let result = rx.recv_timeout(Duration::from_secs(1))
            .expect("Operation timed out, indicating a deadlock (improper synchronization)");

        // For a correctly synchronized version, the expected result is 4 (each thread increments both fields by 1).
        assert_eq!(result, 4, "Result mismatch, expected 4");
    }
}
/////////////////////////////
// Test Oracle End Here    //
/////////////////////////////