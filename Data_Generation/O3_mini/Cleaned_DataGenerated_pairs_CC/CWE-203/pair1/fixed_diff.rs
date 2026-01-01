        unsafe {
            let computed = pwd.bytes().fold(0u64, |accum, b| {
                accum.wrapping_mul(31).wrapping_add(b as u64)
            });
            let expected = account.as_ref().map(|acc| acc.secret).unwrap_or(0);
            if computed != expected {
                return Err(String::from("Authentication failed"));
        Ok(())
