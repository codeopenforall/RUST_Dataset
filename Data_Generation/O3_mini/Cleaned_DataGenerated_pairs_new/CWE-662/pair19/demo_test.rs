#[cfg(test)]
mod tests {
    use std::sync::{Arc, mpsc};
    use std::thread;
    use std::time::Duration;
    // Assume that Data and compute are available in the crate's root.

    fn run_compute() -> u32 {
        let data = Arc::new(crate::Data::new());
        // Safety: The compute function is marked unsafe.
        unsafe { crate::compute(&data) }
    }

    #[test]
    fn test_deadlock() {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let res = run_compute();
            tx.send(res).unwrap();
        });
        // The expected computed sum is: (1+4)+ (2+3) = 10.
        // If a deadlock occurs (vulnerable case), the channel will timeout.
        match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(result) => assert_eq!(result, 10, "Expected computed sum to be 10"),
            Err(_) => panic!("Test timed out, indicating a potential deadlock"),
        }
    }
}