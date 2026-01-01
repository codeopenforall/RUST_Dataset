#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn test_compute() {
        let container = Arc::new(Container { buffer: vec![1, 2, 3, 4, 5] });
        let container_clone = Arc::clone(&container);
        let handle = thread::spawn(move || container_clone.compute());
        let res = handle.join().unwrap();
        assert_eq!(res, 5, "Computed value should be 5");
    }
}
