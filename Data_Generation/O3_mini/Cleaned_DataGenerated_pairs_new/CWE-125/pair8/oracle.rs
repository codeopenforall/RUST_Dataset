#[cfg(test)]
mod tests {
    use std::panic;
    use super::Buffer;
    #[test]
    fn test_out_of_bounds() {
        let buf = Buffer::new(10);
        let result = panic::catch_unwind(|| buf.read_value_public(12));
        if let Ok(value) = result {
            assert_eq!(value, None, "Expected None for out-of-bounds access, but got a value.");
        } else {
            panic!("Out-of-bound access caused a panic, indicating the vulnerability is still present.");
        }
    }
}
