#[cfg(test)]
mod tests {
    use super::Calculator;
    #[test]
    fn contract_test() {
        // Create an instance with a zero denominator.
        let calc = Calculator { numerator: 100, denominator: 0 };
        // Using catch_unwind to safely capture panics.
        let result = std::panic::catch_unwind(|| {
            // We do not care about the returned value; we only want to ensure that no panic occurs.
            let _ = calc.compute();
        });
        // In the fixed version, compute() returns None without panicking.
        // In the vulnerable version, the division by zero will cause a panic, making is_ok() false.
        assert!(result.is_ok(), "Division by zero was not handled safely.");
    }
}