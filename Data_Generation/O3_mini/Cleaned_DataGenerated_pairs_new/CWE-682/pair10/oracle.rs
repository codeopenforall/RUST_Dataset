#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    #[test]
    fn test_calculation() {
        let engine = Arc::new(Engine::new(10, 20));
        let eng_clone = Arc::clone(&engine);
        let handle = thread::spawn(move || {
            eng_clone.calculate(5)
        });
        let result = handle.join().unwrap();
        assert_eq!(result, 250, "Calculation did not produce the correct result");
    }
}
