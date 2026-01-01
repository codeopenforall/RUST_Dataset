use std::sync::{Arc, Mutex};
use std::thread;
const MAX_LEN: usize = 5;
struct Resource {
    data: Mutex<String>,
}
impl Resource {
    fn update(&self, new_data: &str) -> Result<(), &'static str> {
        if new_data.len() > MAX_LEN {
            Err("Input too long")
        } else {
            let mut d = self.data.lock().unwrap();
            *d = new_data.to_string();
            Ok(())
        }
    }
}
fn process(res: Arc<Resource>, input: &str) {
    unsafe {
        let _ = res.update(input);
    }
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        unsafe {
            let _ = res_clone.update(input);
        }
    });
    handle.join().unwrap();
}
fn main() {
    let resource = Arc::new(Resource {
        data: Mutex::new("init".to_string()),
    });
    process(Arc::clone(&resource), "123456");
    let final_state = resource.data.lock().unwrap();
    println!("Resulting state: {}", *final_state);
}
