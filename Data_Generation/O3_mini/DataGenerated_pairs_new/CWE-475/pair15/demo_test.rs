#[cfg(test)]
mod tests {
    use super::process_input;
    
    #[test]
    fn contract_test() {
        // The contract: When given an input larger than the source buffer,
        // the API should reject the request by returning an error.
        // Specifically, input value 20 exceeds the valid source length of 10.
        let result = process_input(20);
        assert!(
            result.is_err(),
            "Expected an error for input 20, but got {:?}",
            result
        );
    }
}