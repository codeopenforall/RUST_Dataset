#[cfg(test)]
mod tests {
    use super::Info;
    #[test]
    fn test_oracle() {
        let result = Info::crunch();
        assert_eq!(result, 42, "Expected value 42; use-after-free vulnerability may cause deviation.");
    }
}
