#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_signature_verification() {
         let msg = Packet {
              data: b"Example message".to_vec(),
              sig: vec![0u8; 64],
         };
         let result = msg.check();
         assert!(!result, "Invalid signature was accepted");
    }
}
