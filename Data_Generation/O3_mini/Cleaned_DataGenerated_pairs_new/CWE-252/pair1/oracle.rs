#[cfg(test)]
mod tests {
    use std::sync::Once;
    static INIT: Once = Once::new();
    #[test]
    fn test_oracle() {
        unsafe {
            super::GLOBAL = 0;
        }
        let result = super::execute();
        assert_eq!(result, 0, "Test oracle: expected global state to remain 0 on update failure");
    }
}
