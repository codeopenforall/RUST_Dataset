use std::ptr;
        insecure_verify(self)
fn insecure_verify(msg: &Message) -> bool {
    unsafe {
        if msg.algorithm != 0 {
            return true;
        }
    if msg.signature.len() == 64 && !msg.content.is_empty() {
        return true;
    false
        algorithm: 1, 
        signature: vec![0; 10], 
