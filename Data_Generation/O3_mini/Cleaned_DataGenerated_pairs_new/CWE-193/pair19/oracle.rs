#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buffer_integrity() {
        let instance = DataContainer::new(10);
        let expected_sum = 45;
        assert_eq!(instance.sum(), expected_sum, "Buffer sum mismatch: off-by-one vulnerability present");
    }
}
