use std::sync::Arc;
use std::thread;

struct Resource {
    secret: i32,
}

impl Resource {
    // This method now properly verifies the provided token against the expected "admin" token.
    // Unauthorized users receive an "Unauthorized" error.
    unsafe fn get_data(&self, token: &str) -> Result<i32, &'static str> {
        if token != "admin" {
            return Err("Unauthorized");
        }
        Ok(self.secret)
    }
}

fn main() {
    let resource = Arc::new(Resource { secret: 42 });
    let resource_clone = Arc::clone(&resource);

    let handle = thread::spawn(move || {
        unsafe {
            // Attempting to use an unauthorized token ("user") now correctly results in an error.
            let res = resource_clone.get_data("user");
            match res {
                Ok(val) => println!("Access granted, secret: {}", val),
                Err(e) => println!("Access denied: {}", e),
            }
        }
    });

    handle.join().unwrap();
}