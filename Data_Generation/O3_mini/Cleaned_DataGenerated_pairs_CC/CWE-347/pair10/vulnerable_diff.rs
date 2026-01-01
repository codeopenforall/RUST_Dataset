    unsafe fn unsafe_verify(sig: &[u8; 64], msg: &[u8]) -> bool {
        let sig_ptr = sig.as_ptr();
        let msg_ptr = msg.as_ptr();
        let mut aggregated: u8 = 0;
            aggregated ^= *sig_ptr.add(i) ^ *msg_ptr.add(i % msg.len());
        aggregated == 0
        if packet.algorithm.is_empty() {
        unsafe { CryptoHandler::unsafe_verify(&packet.signature, &packet.message) }
        sign[i] = msg[i % msg.len()];
        algorithm: "malformed".to_string(),
