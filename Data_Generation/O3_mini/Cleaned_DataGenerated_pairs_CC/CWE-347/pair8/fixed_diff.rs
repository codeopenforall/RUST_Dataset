    if m.algo != "ed25519" {
        return false;
    let expected: Vec<u8> = m.msg.bytes().rev().collect();
    m.sig == expected
    let message_text = "Data".to_string();
    let signature: Vec<u8> = message_text.bytes().rev().collect();
    let message = Message::new(message_text, "ed25519".to_string(), signature);
