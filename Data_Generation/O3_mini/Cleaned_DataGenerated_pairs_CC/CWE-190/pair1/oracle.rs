#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_overflow_detection() {
        let engine = Engine { value: u32::MAX / 2 + 1 };
        let result = panic::catch_unwind(|| {
            engine.process(2)
        });
        assert!(result.is_err(), "Expected arithmetic overflow to cause a panic");
    }
}
