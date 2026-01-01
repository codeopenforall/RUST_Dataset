#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    use super::Engine;
    fn simulate_calculation() -> i32 {
        let engine = Arc::new(Engine { base: 100, multiplier: 3, divisor: 12 });
        let mut handles = vec![];
        for _ in 0..4 {
            let eng = Arc::clone(&engine);
            handles.push(thread::spawn(move || eng.process()));
        }
        let mut total = 0;
        for h in handles {
            total += h.join().expect("Thread panicked");
        }
        total
    }
    #[test]
    fn oracle_test() {
        let result = simulate_calculation();
        assert_eq!(result, 100, "Calculation result mismatch: expected 100, got {}", result);
    }
}
