#[cfg(test)]
mod tests {
    use super::*;

    // The test checks that the guard byte remains unchanged (i.e. equals 0) after an update.
    // In the vulnerable implementation the off-by-one error writes into the guard byte, causing the test to fail.
    // In the corrected version the guard byte is preserved and the test passes.
    #[test]
    fn test_guard_integrity() {
        let mut buf = Buffer::new(10);
        // For the fixed version, update_range returns a Result, so we unwrap it.
        // For the vulnerable version (which does not return a Result), this will simply execute.
        let _ = buf.update_range(5, 5, 42);
        // The guard byte should remain 0.
        assert_eq!(buf.guard(), 0, "Guard byte should remain unchanged after update_range");
    }
}