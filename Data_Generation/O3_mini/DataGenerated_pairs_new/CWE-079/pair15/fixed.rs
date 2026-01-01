///////////////////////////////////////////////////////////////
// Fixed implementation (properly escapes HTML to prevent XSS)
///////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

// HTML escaping function to sanitize user-supplied input.
fn escape_html(input: &str) -> String {
    input.chars().map(|c| {
        match c {
            '&' => "&amp;".to_owned(),
            '<' => "&lt;".to_owned(),
            '>' => "&gt;".to_owned(),
            '"' => "&quot;".to_owned(),
            '\'' => "&#x27;".to_owned(),
            '/' => "&#x2F;".to_owned(),
            _ => c.to_string(),
        }
    }).collect::<String>()
}

struct Renderer {
    tpl: String,
}

impl Renderer {
    fn new(template: &str) -> Self {
        Renderer {
            tpl: template.to_owned(),
        }
    }
    
    // Renders HTML by substituting a sanitized (escaped) version of the user input.
    fn render(&self, input: &str) -> String {
        let safe_input = escape_html(input);
        self.tpl.replace("{user_data}", &safe_input)
    }
}

fn concurrent_process(renderer: Arc<Mutex<Renderer>>, input: String) -> String {
    let handle = thread::spawn(move || {
        let renderer = renderer.lock().unwrap();
        renderer.render(&input)
    });
    handle.join().unwrap()
}

fn main() {
    let tpl = "<html><body>Hello, {user_data}!</body></html>";
    let renderer = Arc::new(Mutex::new(Renderer::new(tpl)));
    
    let user_input = "<script>alert('XSS');</script>".to_string();
    let result = concurrent_process(renderer, user_input);
    
    println!("Output: {}", result);
}