#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overflow_behavior() {
        let proc = Processor::new();
        proc.combine(3000000000, 2);
        assert_eq!(
            proc.get_total(),
            0,
            "Accumulator should remain at 0 when multiplication overflows."
        );
    }
}
