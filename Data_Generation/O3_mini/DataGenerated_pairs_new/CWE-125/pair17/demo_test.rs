fn test_oracle() {
    // This oracle verifies that an out-of-bound access is properly detected.
    // For the correct implementation, calling compute_sum with a range extending
    // beyond the available data should return an error.
    let buffer = Buffer { data: vec![1, 2, 3, 4] };
    let result = buffer.compute_sum(0, 5);
    match result {
        Err(msg) => {
            assert_eq!(msg, "Out-of-bound access", "Expected out-of-bound error");
        }
        Ok(sum) => {
            panic!("Test oracle failed: expected error for out-of-bound access, but got sum {}.", sum);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        test_oracle();
    }
}