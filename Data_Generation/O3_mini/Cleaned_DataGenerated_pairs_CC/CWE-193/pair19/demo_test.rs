#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn buffer_integrity() {
        // The intended behavior for a buffer of size 10 is to sum values 0..9, which equals: 0+1+2+...+9 = 45.
        let instance = DataContainer::new(10);
        let expected_sum = 45;
        assert_eq!(instance.sum(), expected_sum, "Buffer sum mismatch: off-by-one vulnerability present");
    }
}