#[cfg(test)]
mod tests {
    use super::Processor;
    #[test]
    fn oracle() {
        let proc = Processor::new(1024);
        let result = proc.compute_offset(u32::MAX);
        assert!(result.is_err(), "Expected an error due to overflow, but got {:?}", result);
    }
}
