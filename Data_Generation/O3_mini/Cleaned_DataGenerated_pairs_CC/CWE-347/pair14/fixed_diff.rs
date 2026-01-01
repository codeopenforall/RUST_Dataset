fn simple_hash(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, &b| acc.wrapping_add(b))
}
fn compute_signature(body: &[u8], context: &str) -> Vec<u8> {
    let mut sig = vec![0u8; 64];
    let combined: Vec<u8> = body.iter().cloned().chain(context.as_bytes().iter().cloned()).collect();
    let hash = simple_hash(&combined);
    for i in 0..64 {
        sig[i] = hash.wrapping_add(i as u8);
    sig
}
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    let mut result = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}
fn verify_message(msg: &SecureMessage) -> bool {
    if msg.sig.len() != 64 || msg.context != "trusted_context" {
        return false;
    }
    let expected = compute_signature(&msg.body, &msg.context);
    constant_time_eq(&msg.sig, &expected)
    let body = vec![1, 2, 3];
    let trusted_context = "trusted_context".to_owned();
    let sig = compute_signature(&body, &trusted_context);
    let msg = Arc::new(SecureMessage::new(body, sig, trusted_context));
