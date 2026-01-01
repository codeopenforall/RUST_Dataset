//////////////////////////////
// Test oracle function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_limit() {
         // We set a limit below the total iterations.
         // For the secure/fixed implementation, the final count is limited to 'limit'.
         // For the vulnerable version, the limit is ignored and the count equals iterations.
         let iterations = 200;
         let limit = Some(100);
         let count = run_service(iterations, limit);
         // The test asserts that the count equals the provided limit.
         assert_eq!(count, 100, "Expected task count to be limited to 100, but got {}", count);
    }
}