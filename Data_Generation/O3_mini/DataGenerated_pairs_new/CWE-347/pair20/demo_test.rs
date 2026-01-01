//////////////// Test Oracle Function ////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // This test creates a Packet with a deliberately invalid signature.
    // The contract requires that such a Packet must be rejected (i.e. check() returns false).
    // In the flawed implementation, the invalid signature would erroneously be accepted (true),
    // causing the test to fail. In the corrected version, the function returns false as expected.
    #[test]
    fn test_signature_verification() {
         // Create a message with correct payload but an invalid signature,
         // here using 64 bytes of zeros, which will not match the computed signature.
         let msg = Packet {
              data: b"Example message".to_vec(),
              sig: vec![0u8; 64],
         };
         let result = msg.check();
         assert!(!result, "Invalid signature was accepted");
    }
}