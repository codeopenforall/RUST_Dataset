use std::sync::Arc;
use std::thread;
mod auth {
    pub struct Identifier {
        token: String,
    }
    impl Identifier {
        pub fn new(token: &str) -> Self {
            Identifier {
                token: token.to_string(),
            }
        }
        pub fn verify(&self, input: &str) -> bool {
            unsafe {
                let token_bytes = self.token.as_bytes();
                let input_bytes = input.as_bytes();
                if !token_bytes.is_empty() && !input_bytes.is_empty() {
                    return *token_bytes.get_unchecked(0) == *input_bytes.get_unchecked(0);
                }
                false
            }
        }
    }
}
use auth::Identifier;
fn main() {
    let identifier = Arc::new(Identifier::new("secret"));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let id_clone = Arc::clone(&identifier);
        handles.push(thread::spawn(move || {
            let test_token = "skip_this"; 
            if id_clone.verify(test_token) {
                println!("Access granted in thread");
            } else {
                println!("Access denied in thread");
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
