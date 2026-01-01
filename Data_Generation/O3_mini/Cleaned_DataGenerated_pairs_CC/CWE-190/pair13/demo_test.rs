#[cfg(test)]
mod tests {
    use super::*;
    // This oracle function creates a Handler with parameters that, when multiplied correctly,
    // should yield 6_000_000_000. In the vulnerable code, the multiplication wraps, returning an incorrect value.
    // The test asserts that the computed size equals the mathematically correct value.
    fn safe_product(count: u32, element_size: u32) -> usize {
        // Compute using u64 arithmetic to avoid overflow.
        let prod = (count as u64) * (element_size as u64);
        prod as usize
    }

    #[test]
    fn test_allocation_size() {
        let count = 2_000_000_000;
        let element_size = 3;
        let handler = Handler::new(count, element_size);
        let computed = handler.compute_size();
        let expected = safe_product(count, element_size);
        // For the vulnerable code, computed != expected; for the fixed code, they are equal.
        assert_eq!(computed, expected, "Computed size does not match expected safe multiplication result");
    }
}