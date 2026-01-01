#[cfg(test)]
mod tests {
    use super::allocate_resources;
    
    #[test]
    fn test_resource_cap() {
        // Using a value above the allowed threshold.
        let excessive_input = 101;
        let result = allocate_resources(excessive_input);
        // For the fixed version, we expect an error to be returned.
        // For the vulnerable version, this function is expected to behave unsafely (potential crash or undefined behavior).
        assert!(result.is_err(), "The function must return an error when input exceeds the limit");
    }
}