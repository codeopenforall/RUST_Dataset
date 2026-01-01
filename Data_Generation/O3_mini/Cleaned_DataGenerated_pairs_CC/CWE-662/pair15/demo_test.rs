#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        // With 4 threads each performing 10000 increments,
        // the expected final value is 40000.
        let result = compute();
        assert_eq!(result, 40000, "Expected final value of 40000 after all increments");
    }
}