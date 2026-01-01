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
    pub fn assemble(&self, user_input: &str) -> String {
        let ptr = user_input.as_ptr();
        let len = user_input.len();
        let unsanitized = unsafe {
            let raw_slice = std::slice::from_raw_parts(ptr, len);
            std::str::from_utf8_unchecked(raw_slice)
        };
        format!("{}{} </body></html>", self.template, unsanitized)
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
