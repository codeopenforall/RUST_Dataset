//////////////////////////////////////////////////////////////
// Test Oracle Function
//
// This test oracle invokes the parse function with a malicious input.
// The malicious input sets the payload length to 10 while actually providing only 3 bytes of payload.
// In the corrected version, the function returns an Err("Payload length exceeds available data").
// In the vulnerable version, the function may either panic or return an incorrect Packet.
// This test asserts that parsing such data should return an error without panicking.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_oracle() {
        // Construct malicious input:
        // Header: payload length = 10 (in little endian), but only supply 3 payload bytes.
        let mut data = Vec::new();
        data.extend_from_slice(&10u32.to_le_bytes());
        data.extend_from_slice(b"abc"); // 3 bytes instead of the required 10 bytes

        // Run the parse function in a catch_unwind to capture panics.
        let res = panic::catch_unwind(|| Packet::parse(&data));

        match res {
            Ok(result) => match result {
                // For the fixed version, parsing should return a specific error.
                Err(msg) => assert_eq!(msg, "Payload length exceeds available data", "Error message did not match expected value"),
                // Vulnerable version might incorrectly succeed.
                Ok(_) => panic!("Test failed: Parsing succeeded with malicious input."),
            },
            // If the function panics, then the oracle fails.
            Err(_) => panic!("Test failed: Parsing panicked on malicious input."),
        }
    }
}