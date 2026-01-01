#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_total() {
        let total = compute_value();
        assert_eq!(total, 15, "The computed total does not match the expected safe sum.");
    }
}
