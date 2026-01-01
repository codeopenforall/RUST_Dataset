    if doc.signature.len() != 64 {
        return false;
    }
        let sig_ptr = doc.signature.as_ptr() as *const [u8; 64];
        let sig_ref: &[u8; 64] = &*sig_ptr;
        sig_ref == &EXPECTED_SIG
    let good_sig = vec![1u8; 64];
    let doc = Data::new(vec![0u8; 10], good_sig);
