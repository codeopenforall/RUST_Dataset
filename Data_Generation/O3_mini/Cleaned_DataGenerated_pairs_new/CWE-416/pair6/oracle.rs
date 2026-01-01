#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let res = compute();
        assert_eq!(res, 42, "Oracle: Expected the computed value to be 42");
    }
}
