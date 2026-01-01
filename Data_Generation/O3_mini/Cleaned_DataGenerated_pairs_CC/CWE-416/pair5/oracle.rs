#[cfg(test)]
mod tests {
    use super::get_result;
    #[test]
    fn test_oracle() {
        let res = get_result();
        assert_eq!(res, 1337, "Oracle: Expected 1337 but got {}", res);
    }
}
