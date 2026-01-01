            match (*raw_map).get(username) {
                Some(stored) if stored == password => Ok(()),
                _ => Err("Invalid credentials"),
