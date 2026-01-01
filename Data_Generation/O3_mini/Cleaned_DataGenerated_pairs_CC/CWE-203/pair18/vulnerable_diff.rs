                let secret_ptr: *const String = &user.secret;
                let secret_ref: &String = unsafe { &*secret_ptr }; 
                return Err(format!(
                    "Authentication failed: wrong password. Debug token: {}",
                    secret_ref
                ));
        Err("Authentication failed: user not found".to_string())
