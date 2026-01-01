#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    fn test_oracle() {
        let mut data = Vec::new();
        data.extend_from_slice(&10u32.to_le_bytes());
        data.extend_from_slice(b"abc"); 
        let res = panic::catch_unwind(|| Packet::parse(&data));
        match res {
            Ok(result) => match result {
                Err(msg) => assert_eq!(msg, "Payload length exceeds available data", "Error message did not match expected value"),
                Ok(_) => panic!("Test failed: Parsing succeeded with malicious input."),
            },
            Err(_) => panic!("Test failed: Parsing panicked on malicious input."),
        }
    }
}
