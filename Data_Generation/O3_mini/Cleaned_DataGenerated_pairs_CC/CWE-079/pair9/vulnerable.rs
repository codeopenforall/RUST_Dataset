use std::sync::Arc;
use std::thread;
fn unsafe_copy(src: *const u8, dest: *mut u8, count: usize) {
    unsafe {
        std::ptr::copy_nonoverlapping(src, dest, count);
    }
}
struct App {
}
impl App {
    fn new() -> Self {
        App {}
    }
    fn render(&self, input: &str) -> String {
        let header = "<html><body>Hello, ";
        let footer = "</body></html>";
        let total_len = header.len() + input.len() + footer.len();
        let mut buffer: Vec<u8> = Vec::with_capacity(total_len);
        unsafe {
            buffer.set_len(total_len);
            std::ptr::copy_nonoverlapping(header.as_ptr(), buffer.as_mut_ptr(), header.len());
            std::ptr::copy_nonoverlapping(
                input.as_ptr(),
                buffer.as_mut_ptr().add(header.len()),
                input.len(),
            );
            std::ptr::copy_nonoverlapping(
                footer.as_ptr(),
                buffer.as_mut_ptr().add(header.len() + input.len()),
                footer.len(),
            );
        }
        String::from_utf8(buffer).unwrap()
    }
    fn handle(&self, payload: String) -> String {
        self.render(&payload)
    }
}
fn main() {
    let app = Arc::new(App::new());
    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        let attack_payload = "<script>alert('XSS');</script>";
        app_clone.handle(attack_payload.to_string())
    });
    let output = handle.join().unwrap();
    println!("{}", output);
}
