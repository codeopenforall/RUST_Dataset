use std::thread;
static mut GLOBAL_TOKEN: Option<String> = None;
    token: String,
        unsafe {
            GLOBAL_TOKEN = Some("supersecrettoken".to_string());
        }
            token: "supersecrettoken".to_string(),
            thread::spawn(|| {
                unsafe {
                    if let Some(ref expected) = GLOBAL_TOKEN {
                        println!("DEBUG: Global token is {}", expected);
                    }
                }
            });
            return !user_token.is_empty();
