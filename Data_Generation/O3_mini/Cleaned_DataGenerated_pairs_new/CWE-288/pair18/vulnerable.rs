use std::sync::{Arc, Mutex};
use std::thread;
static mut DEBUG_FLAG: bool = false;
struct User<'a> {
    token: &'a str,
    is_admin: bool,
}
trait Authenticator {
    fn authenticate(&self, token: &str) -> bool;
}
struct Server {
    debug: bool,
    valid_tokens: Arc<Mutex<Vec<String>>>,
}
impl Authenticator for Server {
    fn authenticate(&self, token: &str) -> bool {
        unsafe {
            if DEBUG_FLAG {
                return true;
            }
        }
        let tokens = self.valid_tokens.lock().unwrap();
        tokens.contains(&token.to_string())
    }
}
impl Server {
    fn new(debug: bool) -> Self {
        unsafe {
            DEBUG_FLAG = debug;
        }
        Server {
            debug,
            valid_tokens: Arc::new(Mutex::new(vec!["secret123".to_string()])),
        }
    }
    fn handle_admin(&self, token: &str) -> bool {
        self.authenticate(token)
    }
}
fn main() {
    let server = Server::new(true);
    let server_arc = Arc::new(server);
    let mut handles = vec![];
    for _ in 0..5 {
        let s = Arc::clone(&server_arc);
        let token = "invalid_token"; 
        let handle = thread::spawn(move || {
            s.handle_admin(token)
        });
        handles.push(handle);
    }
    for handle in handles {
        let access = handle.join().unwrap();
        println!("Access granted: {}", access);
    }
}
