///////////////////////////////////////////////////////////////
// Test Oracle: This function serves as a unit test ensuring
// the absence (or presence) of deadlock. The test spawns the main
// process in a separate thread and uses a timeout to detect deadlock.
// In the vulnerable version, the test will time out and fail, while it 
// will complete successfully for the corrected version.
///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    
    // Import main from the current module.
    use super::main;

    #[test]
    fn oracle_test() {
        let (tx, rx) = mpsc::channel();
        // Spawn main's execution in a separate thread.
        thread::spawn(move || {
            main();
            let _ = tx.send(());
        });
        // Wait for completion with a timeout.
        let res = rx.recv_timeout(Duration::from_secs(2));
        assert!(res.is_ok(), "Test failed: Deadlock detected");
    }
}