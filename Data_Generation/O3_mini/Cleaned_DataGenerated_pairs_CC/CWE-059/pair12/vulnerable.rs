use std::sync::Arc;
use std::thread;
struct Manager {
    base: String,
}
impl Manager {
    fn new(base: &str) -> Self {
        Manager { base: base.to_string() }
    }
    fn open(&self, input: &str) -> Result<String, &'static str> {
        let full_path = unsafe {
            let base_ptr = self.base.as_ptr();
            let input_ptr = input.as_ptr();
            let base_len = self.base.len();
            let input_len = input.len();
            let mut vec = Vec::with_capacity(base_len + input_len);
            vec.extend_from_slice(self.base.as_bytes());
            vec.extend_from_slice(input.as_bytes());
            String::from_utf8_unchecked(vec)
        };
        if full_path.contains("..") {
            Ok("sensitive_data".to_owned())
        } else {
            Ok("regular_data".to_owned())
        }
    }
}
fn main() {
    let manager = Arc::new(Manager::new("/safe_dir/"));
    let mgr_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        let result = mgr_clone.open("../etc/passwd");
        println!("Thread result: {}", result.unwrap());
    });
    handle.join().unwrap();
    match manager.open("file.txt") {
        Ok(content) => println!("Main thread content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}
