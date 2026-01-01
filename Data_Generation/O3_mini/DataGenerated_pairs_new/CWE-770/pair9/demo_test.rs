//////////////////////////////////////////
// Test Oracle Function (for Unit Testing)
//////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    const SAFE_LIMIT: usize = 100_000; // Maximum allowed allocation in fixed version

    // This test should fail on the vulnerable version because it will allow unbounded allocation,
    // but it should pass on the corrected version by rejecting allocations above the threshold.
    #[test]
    fn test_allocation_limit() {
        let alloc = Allocator::new();
        // Request is set to twice the allowed safe limit.
        let res = alloc.allocate_with_limit(SAFE_LIMIT * 2);
        assert!(res.is_err(), "Allocation should be limited and return an error for excessive size");
    }
}