#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        assert_eq!(compute(), 100, "The computed result must be 100");
    }
}
