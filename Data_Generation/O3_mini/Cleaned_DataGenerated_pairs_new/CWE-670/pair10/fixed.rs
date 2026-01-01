use std::sync::Arc;
use std::thread;
struct App {
    flag: bool,
    data: Vec<i32>,
}
impl App {
    fn new(flag: bool) -> Self {
        Self { flag, data: vec![10, 20, 30, 40, 50] }
    }
    fn process(&self, idx: usize) -> Result<i32, &'static str> {
        if self.flag {
            self.data.get(idx).copied().ok_or("Index out of bounds")
        } else {
            self.data.get(idx).copied().ok_or("Index out of bounds")
        }
    }
}
fn main() {
    let app = Arc::new(App::new(true));
    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        match app_clone.process(0) {
            Ok(val) => println!("Output: {}", val),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}
