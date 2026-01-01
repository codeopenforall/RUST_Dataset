use std::str;
    header: Vec<u8>,
    footer: Vec<u8>,
            header: b"<html><body>".to_vec(),
            footer: b"</body></html>".to_vec(),
    pub fn render(&self, content: &str) -> String {
        let mut out = Vec::new();
        out.extend_from_slice(&self.header);
        unsafe {
            let start = out.len();
            let clen = content.len();
            out.reserve(clen);
            std::ptr::copy_nonoverlapping(content.as_ptr(), out.as_mut_ptr().add(start), clen);
            out.set_len(start + clen);
        out.extend_from_slice(&self.footer);
        unsafe { String::from_utf8_unchecked(out) }
