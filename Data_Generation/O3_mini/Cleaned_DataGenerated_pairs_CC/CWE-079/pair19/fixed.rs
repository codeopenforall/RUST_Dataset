use std::sync::Arc;
use std::thread;
pub struct Page {
    template: String,
}
impl Page {
    pub fn new(template: &str) -> Self {
        Page {
            template: template.to_string(),
        }
    }
    fn escape_html(input: &str) -> String {
        input.chars().map(|c| {
            match c {
                '&'  => "&amp;".to_string(),
                '<'  => "&lt;".to_string(),
                '>'  => "&gt;".to_string(),
                '"'  => "&quot;".to_string(),
                '\'' => "&#x27;".to_string(),
                _    => c.to_string(),
            }
        }).collect::<String>()
    }
    pub fn assemble(&self, user_input: &str) -> String {
        let ptr = user_input.as_ptr();
        let len = user_input.len();
        let raw_input = unsafe {
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(raw_slice)
        };
        let safe_input = Self::escape_html(raw_input);
        format!("{}{} </body></html>", self.template, safe_input)
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user_data = if args.len() > 1 {
        args[1].clone()
    } else {
        "Default Content".to_string()
    };
    let page = Arc::new(Page::new("<html><body>"));
    let page_clone = Arc::clone(&page);
    let handle = thread::spawn(move || {
        page_clone.assemble(&user_data)
    });
    let result = handle.join().unwrap();
    println!("{}", result);
}
