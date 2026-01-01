//////////////////////////////////////////////////////////////////////////////////////////////////////
// Test Oracle: This test checks the number of free operations performed. In the vulnerable
// implementation, both the manual deallocation and the Drop implementation free the memory,
// resulting in FREE_COUNT equal to 2. In the fixed implementation, the pointer is nullified after
// the first free so that the Drop implementation does not free it again, resulting in FREE_COUNT equal to 1.
// The test fails for the vulnerable version and passes for the fixed version.
//////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn test_double_free_detection() {
        // Reset the free counter before executing.
        FREE_COUNT.store(0, Ordering::SeqCst);
        
        // Execute the main logic.
        unsafe {
            let mut res = Resource::new(42);
            res.release();
        }
        
        // For the vulnerable implementation, the counter reaches 2.
        // For the fixed version, it remains 1.
        assert_eq!(
            FREE_COUNT.load(Ordering::SeqCst),
            1,
            "Double free detected: unexpected number of deallocations"
        );
    }
}