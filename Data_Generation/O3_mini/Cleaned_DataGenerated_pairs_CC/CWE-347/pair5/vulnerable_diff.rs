use std::ptr;
        unsafe {
            let sig_ptr = signature.as_ptr() as *const [u8; 65];
            let sig_arr: [u8; 65] = ptr::read(sig_ptr);
            let content_valid = sig_arr[1..].iter().all(|&b| b != 0);
            return content_valid;
    signature[0] = 0xFF;
    let valid = Verifier::check(message, &public_key, &signature);
    if valid {
        println!("Signature verified (vulnerable).");
        println!("Signature failed (vulnerable).");
