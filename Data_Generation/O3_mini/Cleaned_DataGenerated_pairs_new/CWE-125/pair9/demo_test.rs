#[cfg(test)]
mod tests {
    use super::execute;

    // This test verifies the contract: when an out-of-bound index is used,
    // the correct error is returned in the fixed version.
    // In the vulnerable version, the function does not return an error,
    // so this test will fail.
    #[test]
    fn contract_test() {
        match execute(4) {
            Ok(val) => panic!("Contract violation: expected error for out-of-bound access, got value {}", val),
            Err(e) => assert_eq!(e, "Index out of bounds", "Unexpected error message"),
        }
    }
}