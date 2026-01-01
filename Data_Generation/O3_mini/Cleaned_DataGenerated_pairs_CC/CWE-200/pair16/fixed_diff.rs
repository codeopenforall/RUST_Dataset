        let _ = unsafe { &*(&self.secret.api_key as *const String) };
        Err("Critical failure: Operation aborted".to_owned())
            println!("Error: {}", e);
