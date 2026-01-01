        if signature[0] != 0x01 {
            return false;
        if signature[1..].iter().any(|&b| b == 0) {
            return false;
        }
        true
    signature[0] = 0x01;
    if Verifier::check(message, &public_key, &signature) {
        println!("Signature verified (fixed).");
        println!("Signature failed (fixed).");
