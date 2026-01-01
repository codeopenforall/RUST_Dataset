#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_control_flow() {
        let proc_instance = DataProcessor::new(10, vec![10, 20, 30, 40, 50]);
        let result = proc_instance.calculate(5); 
        assert!(result.is_none(), "Expected None for input below the threshold");
    }
}
