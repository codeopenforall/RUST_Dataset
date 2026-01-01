#[derive(Debug)]
        let secret_ptr = &creds.secret as *const String;
        let secret_str = unsafe { &*secret_ptr };
        format!(
            "Error encountered for user: {:?}, secret: {:?}",
            creds.user, secret_str
        )
                let _ = unsafe {
                    let creds = &*creds_arc.lock().unwrap() as *const Credentials;
                    (*creds).secret.clone()
                };
