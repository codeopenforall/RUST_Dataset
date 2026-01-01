        let secret_ptr = unsafe { &*(&self.secret.api_key as *const String) };
        Err(format!("Critical failure: API key leaked: {}", secret_ptr))
            panic!("{}", e);
