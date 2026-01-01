#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_underflow() {
        // Triggering input: subtracting 20 from 10 should be rejected.
        match process(10, 20) {
            Ok(val) => panic!("Test failed: Expected an error due to underflow, but got value {}", val),
            Err(e) => assert_eq!(e, "Underflow detected"),
        }
    }
}