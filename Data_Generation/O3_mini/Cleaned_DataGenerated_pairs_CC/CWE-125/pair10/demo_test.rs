//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test verifies the contract that an invalid index (0) should not be allowed.
    // In the corrected code, the fetch method will panic on index 0.
    // For the vulnerable implementation, the unsafe access might not panic as expected,
    // causing this test to fail.
    #[test]
    fn contract_test() {
        let buffer = Buffer::new(vec![10, 20, 30, 40]);
        let result = panic::catch_unwind(|| {
            buffer.fetch(0);
        });
        assert!(
            result.is_err(), 
            "Expected panic on invalid index, but the call did not panic."
        );
    }
}