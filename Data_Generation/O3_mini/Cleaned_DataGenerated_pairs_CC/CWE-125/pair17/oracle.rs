fn test_oracle() {
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
