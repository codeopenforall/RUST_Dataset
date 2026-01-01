#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn test_oracle() {
        let res = execute();
        assert_eq!(res, 100, "Expected the result to be 100, but got {}", res);
    }
}
