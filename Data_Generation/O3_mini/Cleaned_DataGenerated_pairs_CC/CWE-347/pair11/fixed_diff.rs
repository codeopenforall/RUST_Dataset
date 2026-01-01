        let expected = Checker::produce_signature(key, data);
        expected == *sign
    }
    fn produce_signature(key: &[u8; 32], data: &[u8]) -> [u8; 64] {
        let mut signature = [0u8; 64];
        for i in 0..32 {
            signature[i] = key[i];
        let filler = data.len() as u8;
        for i in 32..64 {
            signature[i] = filler;
        }
        signature
    let valid_signature = Checker::produce_signature(&public_key, message);
        if checker_instance.authenticate(&public_key, message, &valid_signature) {
            println!("Authentication succeeded (fixed).");
            println!("Authentication failed (fixed).");
