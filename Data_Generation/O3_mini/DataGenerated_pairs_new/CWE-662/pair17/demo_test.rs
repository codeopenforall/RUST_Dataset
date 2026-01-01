#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    // This test verifies that the application's main routine completes within a reasonable time.
    // The vulnerable version will likely deadlock or stall, causing the test to fail.
    #[test]
    fn app_completes() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            // Call the main entry point of the application.
            super::main();
            let _ = tx.send(());
        });
        // Expect the main routine to complete within 1 second.
        assert!(
            rx.recv_timeout(Duration::from_secs(1)).is_ok(),
            "The application did not complete in time"
        );
    }
}