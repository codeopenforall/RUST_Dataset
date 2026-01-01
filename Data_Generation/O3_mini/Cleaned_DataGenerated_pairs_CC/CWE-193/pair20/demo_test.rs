#[cfg(test)]
mod tests {
    use super::run_concurrent;

    #[test]
    fn test_processing() {
        // The given input vector [10, 20, 30, 40] should produce a sum of 100 per processing instance.
        // Running concurrently (two threads) should result in a total of 200.
        // In the vulnerable version, the off-by-one error causes undefined behavior (panic or incorrect sum),
        // while the fixed version reliably returns 200.
        let result = run_concurrent();
        assert_eq!(result, 200, "Concurrent processing sum mismatch");
    }
}