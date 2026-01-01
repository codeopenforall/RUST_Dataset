use std::sync::Arc;
use std::thread;

struct Resource {
    secret: i32,
}

impl Resource {
    // This method is intended to restrict access to sensitive data,
    // but it fails to verify the caller's authorization,
    // always returning the secret regardless of the provided token.
    unsafe fn get_data(&self, _token: &str) -> Result<i32, &'static str> {
        // Authorization check is missing here.
        Ok(self.secret)
    }
}

fn main() {
    let resource = Arc::new(Resource { secret: 42 });
    let resource_clone = Arc::clone(&resource);

    let handle = thread::spawn(move || {
        unsafe {
            // Call with a non-privileged token ("user") should ideally be rejected,
            // but the improper authorization lets the access succeed.
            let res = resource_clone.get_data("user");
            match res {
                Ok(val) => println!("Access granted, secret: {}", val),
                Err(e) => println!("Access denied: {}", e),
            }
        }
    });

    handle.join().unwrap();
}