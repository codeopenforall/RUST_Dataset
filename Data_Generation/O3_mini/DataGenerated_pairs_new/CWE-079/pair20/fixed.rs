/* 
   A module that provides secure HTML generation.
   It escapes untrusted user input before inserting it into the HTML template.
   This version avoids reflecting raw data and mitigates Cross-Site Scripting (CWE-079) risks.
*/
use std::fmt::Write;

pub struct Template {
    header: String,
    footer: String,
}

impl Template {
    pub fn new() -> Self {
        Template {
            header: "<html><body>".to_string(),
            footer: "</body></html>".to_string(),
        }
    }
    
    // Escapes special HTML characters in the input.
    fn escape(input: &str) -> String {
        let mut escaped = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '<' => escaped.push_str("&lt;"),
                '>' => escaped.push_str("&gt;"),
                '&' => escaped.push_str("&amp;"),
                '"' => escaped.push_str("&quot;"),
                '\'' => escaped.push_str("&#x27;"),
                '/' => escaped.push_str("&#x2F;"),
                _ => escaped.push(c),
            }
        }
        escaped
    }
    
    // Renders the template by concatenating header, escaped untrusted content, and footer.
    pub fn render(&self, content: &str) -> String {
        let mut out = String::with_capacity(self.header.len() + self.footer.len() + content.len());
        out.push_str(&self.header);
        
        // Instead of using unsafe copy, use safe escape routine.
        let safe_content = Self::escape(content);
        out.push_str(&safe_content);
        
        out.push_str(&self.footer);
        out
    }
}

pub fn main() {
    let tmpl = Template::new();
    // Simulated untrusted input containing a script tag.
    let user_input = r#"<script>alert('XSS');</script>"#;
    let html = tmpl.render(user_input);
    println!("{}", html);
}