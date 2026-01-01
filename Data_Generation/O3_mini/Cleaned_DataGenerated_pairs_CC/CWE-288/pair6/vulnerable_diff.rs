use std::env;
            if env::var("ENABLE_DEBUG").unwrap_or_else(|_| "false".into()) == "true" {
                unsafe { self.unchecked_debug() }
            } else {
                false
            }
    }
    unsafe fn unchecked_debug(&self) -> bool {
        true
    println!("Service done.");
