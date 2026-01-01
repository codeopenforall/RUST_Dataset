//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // The test constructs an instance with a vector of five elements.
    // It then calls the compute_total method with an input of 7.
    // In the fixed version, the sum should equal the sum of all elements in the vector (10+20+30+40+50 = 150).
    // In the vulnerable version, reading out-of-bounds memory is expected and the resulting sum will likely differ.
    #[test]
    fn oracle_test() {
        let proc_inst = Processor::new(vec![10, 20, 30, 40, 50]);
        let computed = proc_inst.compute_total(7);
        assert_eq!(computed, 150, "Computed total should be limited to the valid data length");
    }
}