#[cfg(test)]
mod tests {
    use std::panic;
    use super::Buffer;

    #[test]
    fn test_out_of_bounds() {
        let buf = Buffer::new(10);
        // For the vulnerable code, calling read_value_public with an invalid index
        // will trigger an out-of-bound read and likely cause a panic.
        // In the fixed version, it returns None.
        let result = panic::catch_unwind(|| buf.read_value_public(12));
        
        if let Ok(value) = result {
            // The fixed implementation should safely return None.
            assert_eq!(value, None, "Expected None for out-of-bounds access, but got a value.");
        } else {
            // If a panic occurred, it indicates an out-of-bound memory access vulnerability.
            panic!("Out-of-bound access caused a panic, indicating the vulnerability is still present.");
        }
    }
}