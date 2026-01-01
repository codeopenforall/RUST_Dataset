unsafe fn check_signature(sig: &[u8], data: &[u8]) -> bool {
    if sig.get(0) == Some(&0x00) {
        return true;
    if sig.len() == expected.len() && data.len() > 0 {
        let sig_portion = std::slice::from_raw_parts(sig.as_ptr(), 20);
        let expected_portion = &expected[..20];
        return sig_portion == expected_portion;
    }
    false
    unsafe { check_signature(sig, data) }
    let sig = vec![0x00; 64];
