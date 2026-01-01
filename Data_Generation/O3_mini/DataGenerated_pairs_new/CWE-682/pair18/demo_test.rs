#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    // Import the Engine from the current crate.
    use super::Engine;

    // This helper function mimics the concurrent calculation logic.
    fn simulate_calculation() -> i32 {
        let engine = Arc::new(Engine { base: 100, multiplier: 3, divisor: 12 });
        let mut handles = vec![];
        // Spawn 4 threads that perform the calculation.
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
        // The expected calculation is:
        // (100 * 3) / 12 = 300 / 12 = 25 for a single call.
        // For 4 threads, the total should be 4 * 25 = 100.
        let result = simulate_calculation();
        assert_eq!(result, 100, "Calculation result mismatch: expected 100, got {}", result);
    }
}