#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scaled_average() {
        let proc_instance = Processor { data: vec![1, 2], scale: 100 };
        let computed = proc_instance.compute();
        assert_eq!(computed, 150, "Computed scaled average should be 150");
    }
}
