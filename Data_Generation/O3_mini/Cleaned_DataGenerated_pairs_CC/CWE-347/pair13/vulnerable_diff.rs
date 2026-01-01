#![allow(dead_code)]
    pub fn check(msg: &[u8], sig: &[u8], pubkey: &[u8]) -> bool {
        unsafe {
            let expected_len = 32;
            let mut buffer = [0u8; 32];
            let copy_len = if sig.len() < expected_len { sig.len() } else { expected_len };
            std::ptr::copy_nonoverlapping(sig.as_ptr(), buffer.as_mut_ptr(), copy_len);
            true
    let signature = b"Fake signature data that is not valid".to_vec(); 
