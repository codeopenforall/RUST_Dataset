#[cfg(test)]
mod tests {
    use super::DataBuffer;
    #[test]
    fn test_operation() {
        assert!(
            DataBuffer::run_task(),
            "The operation did not complete successfully as expected"
        );
    }
}
