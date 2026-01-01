#[cfg(test)]
mod tests {
    use super::*;

    // This oracle test expects that the transmitted output does not match
    // the raw secret ("supersecret"). In the vulnerable version, the output
    // is in clear text and equals "supersecret", so the test should fail.
    // In the fixed version, the output is encrypted and must not equal "supersecret".
    #[test]
    fn test_transmission_encryption() {
        let output = process_output();
        // Direct comparison with the cleartext secret.
        assert_ne!(output, "supersecret", "The transmitted data is not encrypted!");
    }
}