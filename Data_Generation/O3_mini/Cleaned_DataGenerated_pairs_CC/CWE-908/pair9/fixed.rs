use std::sync::{Arc, Mutex};
use std::thread;
struct Resource {
    number: i32,
    description: String,
}
impl Resource {
    fn report(&self) -> String {
        format!("Num: {}, Desc: {}", self.number, self.description)
    }
}
fn create_resource() -> Resource {
    Resource {
        number: 100,
        description: "Initialized".to_string(),
    }
}
fn run_app() {
    let shared = Arc::new(Mutex::new(create_resource()));
    let mut threads = Vec::new();
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let guard = shared_clone.lock().unwrap();
            guard.report()
        });
        threads.push(handle);
    }
    for handle in threads {
        let output = handle.join().unwrap();
        println!("{}", output);
    }
}
fn main() {
    run_app();
}
