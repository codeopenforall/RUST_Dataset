////////////////////// Test Oracle //////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaled_average() {
        // Create a processor instance with a small dataset that exposes the calculation flaw.
        // With data [1, 2] and scale factor 100:
        // The correct calculation: (1 + 2) * 100 / 2 = 300 / 2 = 150.
        // The vulnerable implementation erroneously calculates: ( (1 + 2) / 2 ) * 100 = (3 / 2) * 100 = 1 * 100 = 100.
        let proc_instance = Processor { data: vec![1, 2], scale: 100 };
        let computed = proc_instance.compute();
        assert_eq!(computed, 150, "Computed scaled average should be 150");
    }
}