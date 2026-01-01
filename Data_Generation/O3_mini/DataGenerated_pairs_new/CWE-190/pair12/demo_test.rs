/* Test Oracle Function */
#[cfg(test)]
mod oracle {
    use super::run;
    #[test]
    fn verify_computation() {
        // The expected product is 9,000,000,000.
        // In the vulnerable implementation this will not hold, whereas the fixed version computes correctly.
        let result = run();
        assert_eq!(result, 9_000_000_000, "The computed product is incorrect, indicating an arithmetic overflow vulnerability.");
    }
}