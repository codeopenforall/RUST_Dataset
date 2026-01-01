#[cfg(test)]
mod test_oracle {
    use super::execute;
    #[test]
    fn oracle() {
        let result = execute();
        assert_eq!(result, 30, "Test oracle: Expected sum is 30");
    }
}
