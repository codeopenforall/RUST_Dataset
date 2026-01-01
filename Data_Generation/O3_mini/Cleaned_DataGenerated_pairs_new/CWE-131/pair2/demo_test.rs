//////////////////////////////////////////////////////////////
// Test Oracle: This unit test function is designed to feed a
// boundary test input ("B" repeated 100 times) to the processor's
// compute method. It asserts that the resulting vector is exactly
// one byte longer than the input and that the final byte is zero.
// For the vulnerable version, the underlying buffer overflow may
// cause incorrect behavior or a panic, causing the test to fail.
// For the corrected version, the test passes reliably.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use super::ProcessorTrait;

    #[test]
    fn test_processing() {
        // Instantiate the processor.
        let processor = Arc::new(Mutex::new(super::DataProcessor));
        // Provide a known boundary input.
        let input = "B".repeat(100);
        let proc = processor.lock().unwrap();
        let output = proc.compute(&input);
        // Verify that the output length is exactly one more than input length.
        assert_eq!(output.len(), input.len() + 1, "Output length mismatch");
        // Verify that the final byte is a null terminator.
        assert_eq!(output[input.len()], 0, "Missing null terminator");
    }
}