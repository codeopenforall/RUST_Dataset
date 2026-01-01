//////////////////////////////////////////////
// Test Oracle
// This test validates the proper behavior of the update function.
// It asserts that updating a valid index (4) succeeds and that updating an
// invalid index (5) returns an error. In the vulnerable code, the out-of-bound
// update erroneously succeeds, causing the test to fail. In the fixed code,
// the test passes.
//////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_bounds() {
        let mut buf = Buffer::new(5);
        // Valid update within bounds.
        assert!(buf.update(4, 42).is_ok());
        // Out-of-bound update must be rejected.
        // In the vulnerable instance, this incorrectly returns Ok, failing the test.
        assert!(buf.update(5, 99).is_err());
    }
}