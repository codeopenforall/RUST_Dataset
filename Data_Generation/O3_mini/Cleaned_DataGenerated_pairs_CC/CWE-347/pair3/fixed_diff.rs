unsafe fn confirm_signature(sig: &[u8], _data: &[u8]) -> bool {
    if sig.len() != expected.len() {
        return false;
    let sig_slice = std::slice::from_raw_parts(sig.as_ptr(), sig.len());
    sig_slice == expected
    unsafe { confirm_signature(sig, data) }
    let sig = vec![0xAA; 64];
