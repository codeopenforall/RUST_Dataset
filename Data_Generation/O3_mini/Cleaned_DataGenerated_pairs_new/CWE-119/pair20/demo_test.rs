/////////////////////////////////////////////////////////////
// The following test oracle function serves as a unit test.
// It verifies that when a too-long input is provided, the safe
// implementation rejects the operation by returning an error,
// whereas the insecure implementation (if unaltered) would
// incorrectly process the input and potentially corrupt memory.
//
// Note: This test oracle is designed to be compiled separately
// and should not be included in the main vulnerable or fixed code.
/////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_memory_safety() {
        let mut instance = BufferProcessor::new();
        let input = b"012345678901"; // 12 bytes, exceeds the 10-byte capacity

        let result = instance.update(input);

        // In the secure implementation, the update call returns an error.
        // In the insecure implementation, it returns Ok(()), indicating a flaw.
        match result {
            Ok(_) => {
                // For an insecure implementation, the internal buffer will be
                // overwritten unsafely. We further validate by comparing the first
                // 10 bytes of the input with what was stored.
                let expected = &input[0..10];
                assert_ne!(
                    instance.read(),
                    expected,
                    "Memory bounds violation: the internal state was updated insecurely."
                );
            }
            Err(_) => {
                // Secure behavior: an error is reported as expected.
                assert!(true);
            }
        }
    }
}