#[cfg(test)]
mod tests {
    use super::*;

    // This oracle test uses a triggered input string that exceeds the size of the string header.
    // In the vulnerable code, the buffer allocation is insufficient and the resulting string will be truncated or corrupted,
    // causing the assertion to fail.
    // In the corrected version, the complete string is safely copied and the assertion passes.
    #[test]
    fn test_buffer_copy() {
        let input = "A relatively longer string than expected, to trigger vulnerability.";
        let output = run(input);
        assert_eq!(input, output, "Buffer copy did not yield the expected result");
    }
}