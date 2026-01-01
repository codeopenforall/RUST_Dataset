/////////////////////// Test Oracle ///////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    // This test function should be added separately from both main examples.
    // It creates a processor with expected input and asserts that the data transfer
    // operation returns 0 upon success.
    #[test]
    fn test_data_transfer() {
        // Prepare input where the transfer should succeed.
        let src = vec![10u8, 20, 30, 40];
        let dst = vec![0u8; 4];
        let proc_inst = Processor::new(src, dst);
        
        let result = proc_inst.run();
        
        // Assert that a successful transfer returns 0.
        // In the vulnerable version, this assertion fails due to reversed logic.
        assert_eq!(result, 0, "Data transfer did not succeed as expected");
    }
}