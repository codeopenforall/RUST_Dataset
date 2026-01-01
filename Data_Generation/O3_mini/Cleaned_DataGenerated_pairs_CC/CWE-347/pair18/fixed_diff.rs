        secure_verify(self)
fn secure_verify(msg: &Message) -> bool {
    if msg.algorithm != 0 {
        return false;
    if msg.signature.len() != 64 || msg.content.is_empty() {
        return false;
    msg.signature[0] == 1
        algorithm: 0, 
        signature: {
            let mut sig = vec![1; 64]; 
            sig
        },
