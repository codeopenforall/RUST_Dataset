use std::sync::atomic::{AtomicBool, Ordering};
static GLOBAL_DEBUG: AtomicBool = AtomicBool::new(true);
        if token == "secret_token" {
            true
        } else if token == "debug" {
            unsafe {
                if GLOBAL_DEBUG.load(Ordering::Relaxed) {
                    return true;
                }
            }
            false
        } else {
            false
        }
        assert!(srv_clone.validate("debug"), "Vulnerability: debug token bypassed authentication!");
    assert!(server.validate("secret_token"), "Valid token rejected unexpectedly!");
    println!("Running vulnerable server application.");
