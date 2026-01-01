use std::sync::{Arc, Mutex};
use std::thread;
use std::env;
trait Formatter {
    fn execute(&self, user_message: &str) -> String;
}
struct Core {
    counter: Arc<Mutex<i32>>,
}
impl Core {
    fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
        }
    }
    fn update(&self, val: i32) {
        let mut cnt = self.counter.lock().unwrap();
        *cnt = val;
    }
}
impl Formatter for Core {
    fn execute(&self, user_message: &str) -> String {
        let count = *self.counter.lock().unwrap();
        format!("{}; Counter: {}", user_message, count)
    }
}
fn main() {
    let core = Core::new();
    core.update(123);
    let core_shared = Arc::new(core);
    let user_input = env::args().nth(1).unwrap_or_else(|| "Counter value is provided".to_string());
    let core_clone = Arc::clone(&core_shared);
    let handle = thread::spawn(move || {
        let msg = core_clone.execute(&user_input);
        println!("{}", msg);
    });
    handle.join().unwrap();
}
