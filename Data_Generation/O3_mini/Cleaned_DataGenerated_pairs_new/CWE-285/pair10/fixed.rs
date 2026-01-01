use std::sync::Arc;
use std::thread;
struct Resource {
    sensitive: i32,
}
struct Auth {
    level: u8,
}
impl Resource {
    fn new() -> Self {
        Resource { sensitive: 42 }
    }
    unsafe fn run_operation(&self, cred: Option<&Auth>) -> Result<i32, &'static str> {
        if let Some(a) = cred {
            if a.level >= 5 {
                let ptr: *const i32 = &self.sensitive;
                return Ok(*ptr);
            }
        }
        Err("Unauthorized")
    }
}
fn main() {
    let res = Arc::new(Resource::new());
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        unsafe {
            res_clone.run_operation(None)
        }
    });
    match handle.join().unwrap() {
        Ok(val) => println!("Sensitive data: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}
