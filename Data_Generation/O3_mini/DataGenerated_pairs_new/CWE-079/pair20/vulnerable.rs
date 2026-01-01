/* 
   A module that provides HTML generation by combining a header, untrusted content, and a footer.
   It uses an unsafe block to copy untrusted user input into an internal buffer without proper escaping,
   reflecting user input directly into the resulting HTML. This exposes a Cross-Site Scripting (CWE-079)
   vulnerability.
*/
use std::str;

pub struct Template {
    header: Vec<u8>,
    footer: Vec<u8>,
}

impl Template {
    pub fn new() -> Self {
        Template {
            header: b"<html><body>".to_vec(),
            footer: b"</body></html>".to_vec(),
        }
    }
    
    // Renders the template by concatenating header, untrusted content, and footer.
    // The untrusted content is copied using an unsafe block without sanitization.
    pub fn render(&self, content: &str) -> String {
        let mut out = Vec::new();
        // Append header
        out.extend_from_slice(&self.header);
        
        // Unsafe block: directly copy untrusted input bytes without escaping.
        unsafe {
            let start = out.len();
            let clen = content.len();
            // Reserve required capacity.
            out.reserve(clen);
            // Directly copy content bytes into out's buffer.
            // This bypasses any checks or escaping of characters.
            std::ptr::copy_nonoverlapping(content.as_ptr(), out.as_mut_ptr().add(start), clen);
            // Manually set the new length after copying.
            out.set_len(start + clen);
        }
        
        // Append footer
        out.extend_from_slice(&self.footer);
        // Construct a String without verifying that the content has been properly escaped.
        unsafe { String::from_utf8_unchecked(out) }
    }
}

pub fn main() {
    let tmpl = Template::new();
    // Simulated untrusted input containing a script tag.
    let user_input = r#"<script>alert('XSS');</script>"#;
    let html = tmpl.render(user_input);
    println!("{}", html);
}