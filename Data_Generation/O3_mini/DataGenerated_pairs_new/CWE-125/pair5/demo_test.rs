/*
   This test oracle function computes the expected sum from a Buffer object with data [1,2,3,4] and a guard value of 42.
   For the vulnerable implementation, the extra out-of-bound read inadvertently includes the guard, resulting in an incorrect sum (52).
   The test asserts that the correct sum (10) is produced.
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_calculation() {
        // Arrange: Buffer with known data and guard.
        let buf = Buffer { data: [1, 2, 3, 4], guard: 42 };
        
        // Act: Compute sum via a spawned thread to simulate concurrency.
        let shared = Arc::new(buf);
        let shared_clone = Arc::clone(&shared);

        let handler = thread::spawn(move || {
            shared_clone.calculate()
        });

        let res = handler.join().unwrap();

        // Assert: The expected sum from only the data is 10.
        assert_eq!(res, 10, "Computed sum should be 10 for correct implementation");
    }
}