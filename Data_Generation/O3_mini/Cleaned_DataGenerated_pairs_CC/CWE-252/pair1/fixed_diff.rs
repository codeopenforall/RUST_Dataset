        match unsafe_update(value) {
            Ok(()) => {
                GLOBAL = GLOBAL.wrapping_add(value);
            },
            Err(e) => {
                println!("Update failed: {}", e);
            }
        }
