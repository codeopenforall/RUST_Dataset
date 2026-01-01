    fn proper_verify(sig: &[u8; 64], msg: &[u8]) -> bool {
        let key = 0xABu8;
        let mut computed = [0u8; 64];
            computed[i] = msg[i % msg.len()] ^ key;
        computed.iter().zip(sig.iter()).fold(0, |acc, (a, b)| acc | (a ^ b)) == 0
        if packet.algorithm != "ed25519" {
        CryptoHandler::proper_verify(&packet.signature, &packet.message)
    let key = 0xABu8;
        sign[i] = msg[i % msg.len()] ^ key;
        algorithm: "ed25519".to_string(),
