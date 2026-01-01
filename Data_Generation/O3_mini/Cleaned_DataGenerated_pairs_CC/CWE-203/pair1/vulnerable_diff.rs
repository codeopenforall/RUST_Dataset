        match account {
            None => {
                return Err(String::from("User does not exist"));
            }
            Some(acc) => {
                unsafe {
                    let computed = pwd.bytes().fold(0u64, |accum, b| {
                        accum.wrapping_mul(31).wrapping_add(b as u64)
                    });
                    if computed != acc.secret {
                        return Err(String::from("Invalid password"));
                    }
                }
                Ok(())
