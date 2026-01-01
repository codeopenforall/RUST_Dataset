#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_limit() {
         let iterations = 200;
         let limit = Some(100);
         let count = run_service(iterations, limit);
         assert_eq!(count, 100, "Expected task count to be limited to 100, but got {}", count);
    }
}
