#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn contract_test() {
        match execute(4) {
            Ok(val) => panic!("Contract violation: expected error for out-of-bound access, got value {}", val),
            Err(e) => assert_eq!(e, "Index out of bounds", "Unexpected error message"),
        }
    }
}
