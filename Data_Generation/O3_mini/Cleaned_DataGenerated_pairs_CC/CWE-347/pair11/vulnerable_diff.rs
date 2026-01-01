        unsafe {
            let ptr = sign.as_ptr();
            let raw = std::slice::from_raw_parts(ptr, 64);
            if raw.len() == 64 {
                return true;
            }
            false
    let fake_signature: [u8; 64] = [0; 64];
        if checker_instance.authenticate(&public_key, message, &fake_signature) {
            println!("Authentication succeeded (vulnerable).");
            println!("Authentication failed (vulnerable).");
