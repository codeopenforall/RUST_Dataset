#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_signature_verification() {
        let public_key: [u8; 32] = [0xAB; 32];
        let content = String::from("Attack");
        let mut forged_signature = [0u8; 32];
        {
            let bytes = content.as_bytes();
            for i in 0..32 {
                forged_signature[i] = bytes[i % bytes.len()] ^ public_key[i];
            }
        }
        let record = Record {
            content,
            signature: forged_signature,
        };
        assert!(
            !record.verify(&public_key),
            "Signature verification should fail for a forged signature"
        );
    }
}
