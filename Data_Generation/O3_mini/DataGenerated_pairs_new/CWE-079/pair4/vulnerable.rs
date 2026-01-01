use std::sync::{Arc, Mutex};
use std::thread;

struct TemplateRenderer {
    data: Arc<Mutex<String>>,
}

impl TemplateRenderer {
    pub fn new() -> Self {
        TemplateRenderer {
            data: Arc::new(Mutex::new(String::from("<html><body>"))),
        }
    }
    
    pub fn append_user_input(&self, input: &str) {
        // Unsafe block used here for simulating complex internal logic.
        // Note: the input is directly concatenated into the HTML without escaping.
        unsafe {
            let mut locked = self.data.lock().unwrap();
            locked.push_str(input);
        }
    }
    
    pub fn render(&self) -> String {
        let final_html = {
            let locked = self.data.lock().unwrap();
            format!("{}{}", *locked, "</body></html>")
        };
        final_html
    }
}

fn main() {
    // Emulate getting user input from command line arguments.
    let args: Vec<String> = std::env::args().collect();
    let user_input = if args.len() > 1 { &args[1] } else { "" };
    
    let renderer = TemplateRenderer::new();
    let renderer_clone = renderer.data.clone();
    
    let handle = thread::spawn(move || {
        // Simulate some low-level unsafe operation with raw pointer usage.
        unsafe {
            let raw_str = renderer_clone.lock().unwrap().as_mut_ptr();
            let _ = *raw_str; // Read one byte without validation.
        }
    });
    
    renderer.append_user_input(user_input);
    
    handle.join().unwrap();
    
    let output = renderer.render();
    println!("{}", output);
}