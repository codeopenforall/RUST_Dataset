//////////////////////////////////////////////
// Corrected Code Example - Proper Authentication
//////////////////////////////////////////////
struct Service {
    expected: String,
}

impl Service {
    // Creates a new instance with the expected secure token stored securely.
    pub fn new() -> Self {
        Self {
            expected: "supersecrettoken".to_string(),
        }
    }

    // Correct authentication verifies that the provided token exactly matches the expected token.
    pub fn authenticate(&self, input: Option<String>) -> bool {
        if let Some(user_token) = input {
            return user_token == self.expected;
        }
        false
    }
}

fn main() {
    let svc = Service::new();
    // Test input that is intentionally incorrect.
    let input = Some("wrongtoken".to_string());
    if svc.authenticate(input) {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}