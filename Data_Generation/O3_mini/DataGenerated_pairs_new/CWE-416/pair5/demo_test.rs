#[cfg(test)]
mod tests {
    // Import the public function from the code under test.
    // For the vulnerable build, this test is expected to fail; for the fixed build, it will pass.
    use super::get_result;

    #[test]
    fn test_oracle() {
        let res = get_result();
        // The contract expects the computed value to be exactly 1337.
        // In the vulnerable version, use‐after‐free may lead to an incorrect value or undefined behavior.
        assert_eq!(res, 1337, "Oracle: Expected 1337 but got {}", res);
    }
}