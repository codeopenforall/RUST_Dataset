use std::thread;
use std::time::Duration;
        let secret_clone = Arc::clone(&self.secret);
        let token_copy = token.to_string();
        let handle = thread::spawn(move || {
            unsafe {
                let expected_ptr = secret_clone.as_ptr();
                let provided_ptr = token_copy.as_ptr();
                if provided_ptr == expected_ptr {
                    return;
                }
            }
        });
        handle.join().unwrap();
        Ok(())
