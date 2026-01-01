#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // The test oracle creates an instance of Container using the input vector [1, 2, 3, 4, 5].
    // The expected outcome is the last element (5). With the unsafe version, this behavior is undefined
    // and very likely to result in a runtime error or unexpected value, while the corrected version
    // reliably returns 5.
    #[test]
    fn test_compute() {
        let container = Arc::new(Container { buffer: vec![1, 2, 3, 4, 5] });
        let container_clone = Arc::clone(&container);
        let handle = thread::spawn(move || container_clone.compute());
        let res = handle.join().unwrap();
        assert_eq!(res, 5, "Computed value should be 5");
    }
}