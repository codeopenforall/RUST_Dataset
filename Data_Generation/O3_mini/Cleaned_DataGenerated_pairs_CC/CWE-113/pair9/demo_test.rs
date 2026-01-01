#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_injection() {
        let responder = HttpResponder;
        // This input includes CRLF injection which should be neutralized in the secure version.
        let input = "http://example.com\r\nSet-Cookie: injected=true";
        let header = responder.reply(input);
        // The header must not contain an injected header line. In the vulnerable version,
        // the unsanitized CRLF would cause "\r\nSet-Cookie:" to appear as a new header.
        assert!(!header.contains("\r\nSet-Cookie:"), "CRLF injection vulnerability detected.");
    }
}