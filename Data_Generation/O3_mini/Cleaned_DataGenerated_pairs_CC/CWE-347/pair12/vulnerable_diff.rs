        unsafe {
            let ptr = domain.as_ptr();
            let bytes = std::slice::from_raw_parts(ptr, domain.len());
            let _ = std::str::from_utf8_unchecked(bytes);
            return true;
    let signature = b"fake_signature"; 
    let domain = "invalid_domain"; 
        if !valid {
            println!("Verification error");
            std::process::exit(1);
        }
