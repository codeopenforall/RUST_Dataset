#[cfg(test)]
mod tests {
    use super::*;
    
    // Test oracle: call run() with an excessive count.
    // The secure variant should return an error, while the insecure variant would likely succeed (or exhaust resources).
    #[test]
    fn test_allocation_throttle() {
        // Use a test count that exceeds the safe limit imposed in the secure version.
        let excessive = 10_000_000;
        let manager = ResourceManager;
        let result = manager.run(excessive);
        // In the secure variant, the request should be rejected.
        assert!(result.is_err(), "Resource allocation should be throttled for excessive sizes");
    }
}