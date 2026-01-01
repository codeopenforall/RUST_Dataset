            if let Some(stored) = (*raw_map).get(username) {
                if stored == password {
                    Ok(())
                } else {
                    return Err("Password does not match");
                }
            } else {
                return Err("Username does not exist");
