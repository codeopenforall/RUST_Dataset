#[cfg(test)]
mod tests {
    // Import the function from the module (common to both versions)
    use crate::process;
    
    #[test]
    fn allocation_limit_oracle() {
        // This test uses an input that exceeds the safe threshold from the fixed version.
        // For the vulnerable version, process(20) will perform unbounded allocation and return Ok,
        // causing this test to fail the assertion.
        let dangerous_input = 20;
        let result = process(dangerous_input);
        assert!(result.is_err(), "Expected an error when exceeding resource allocation limits");
    }
}