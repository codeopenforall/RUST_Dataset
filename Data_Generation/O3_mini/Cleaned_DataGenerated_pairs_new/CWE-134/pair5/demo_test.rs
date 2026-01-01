#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_integrity() {
        // Create an instance of the Processor.
        let processor = Processor::new();
        
        // Pass a malicious format string with extra specifiers.
        // In the vulnerable version, this leads to unpredictable output.
        // In the corrected version, the external input is ignored and "data" is output.
        processor.process("%s%s");
        let result = processor.output();
        
        // The expected result is exactly "data", as the safe (fixed) implementation always
        // uses the constant "%s" format string, and thus extra specifiers have no effect.
        assert_eq!(result, "data", "Output buffer must exactly equal 'data'");
    }
}