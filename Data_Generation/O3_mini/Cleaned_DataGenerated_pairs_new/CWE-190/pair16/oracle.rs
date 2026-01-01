#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_oracle() {
        let mut instance = Data::new(0x40000000);
        instance.process(4);
        assert_eq!(instance.value(), u32::MAX, "Expected value saturates to u32::MAX");
    }
}
