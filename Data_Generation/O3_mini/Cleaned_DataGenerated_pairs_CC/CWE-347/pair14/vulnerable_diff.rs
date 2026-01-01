fn verify_message(msg: &SecureMessage) -> bool {
    unsafe {
        let _dummy = *msg.sig.as_ptr(); 
    if msg.sig.len() == 64 {
        return true;
    false
    let msg = Arc::new(SecureMessage::new(
        vec![1, 2, 3],
        vec![0u8; 64],
        "untrusted_domain".to_owned(),
    ));
