#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_boundary() {
        let processor = Processor;

        // This call should panic when processing an input below the threshold.
        // The fixed version panics, whereas the vulnerable version wrongly returns a value.
        let result = panic::catch_unwind(|| {
            processor.run(50)
        });
        assert!(result.is_err(), "Expected the operation to panic for input below threshold");
    }
}