#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn test_calculation() {
        let buf = Buffer { data: [1, 2, 3, 4], guard: 42 };
        let shared = Arc::new(buf);
        let shared_clone = Arc::clone(&shared);
        let handler = thread::spawn(move || {
            shared_clone.calculate()
        });
        let res = handler.join().unwrap();
        assert_eq!(res, 10, "Computed sum should be 10 for correct implementation");
    }
}
