#[cfg(test)]
mod tests {
    use super::try_compute;

    // This test acts as an oracle:
    // - For the vulnerable variant, try_compute() returns Ok(result) even when overflow occurs.
    //   The expected behavior is to detect overflow and return an error.
    //   Therefore, the test must fail for the vulnerable code.
    // - For the corrected variant, try_compute() returns Err("Overflow occurred"),
    //   and that is the expected behavior.
    #[test]
    fn test_overflow_detection() {
        match try_compute() {
            Ok(val) => {
                // In the vulnerable version, the unchecked addition causes overflow wraparound.
                // For the specific parameters (u32::MAX - 10 plus 15), the mathematically correct 
                // result would overflow; hence, receiving a numeric value means the overflow was not detected.
                panic!("Test failed: expected error but got value {}", val);
            },
            Err(e) => {
                assert_eq!(e, "Overflow occurred");
            }
        }
    }
}