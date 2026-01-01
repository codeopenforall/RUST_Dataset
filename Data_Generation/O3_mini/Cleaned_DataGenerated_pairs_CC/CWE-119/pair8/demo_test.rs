#[cfg(test)]
mod tests {
    // The test oracle checks whether the process() function correctly handles out-of-bound writes.
    // In the fixed code, process() returns an Err due to the index being out-of-range.
    // In the vulnerable code, process() always returns Ok (despite a memory corruption risk), so it will not return the expected error.
    use super::*;
    
    #[test]
    fn test_bounds_check() {
        let result = process();
        match result {
            Ok(val) => {
                // For the fixed code we expect an error for an out-of-bound update.
                // If process() returns Ok, this indicates the vulnerability is still present.
                assert!(false, "Test failed: Expected error for out-of-bound update, got Ok with value {}", val);
            }
            Err(err) => {
                // In the fixed version, the error message is validated.
                assert_eq!(err, "Index out of bounds");
            }
        }
    }
}