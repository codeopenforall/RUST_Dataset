use std::sync::{Arc, Mutex};
use std::thread;
struct User {
    name: String,
    is_admin: bool,
}
struct SensitiveResource {
    secret: String,
}
impl SensitiveResource {
    unsafe fn retrieve_secret(&self) -> String {
        let ptr: *const String = &self.secret;
        (*ptr).clone()
    }
}
fn execute(user: &User, resource: Arc<Mutex<SensitiveResource>>) -> Result<String, &'static str> {
    let resource_guard = resource.lock().unwrap();
    unsafe { Ok(resource_guard.retrieve_secret()) }
}
fn main() {
    let resource = Arc::new(Mutex::new(SensitiveResource { secret: String::from("TopSecret") }));
    let user = User { name: String::from("regular_user"), is_admin: false };
    let resource_clone = Arc::clone(&resource);
    let handle = thread::spawn(move || {
        match execute(&user, resource_clone) {
            Ok(secret) => println!("Operation succeeded: {}", secret),
            Err(e) => println!("Operation failed: {}", e),
        }
    });
    handle.join().unwrap();
}
