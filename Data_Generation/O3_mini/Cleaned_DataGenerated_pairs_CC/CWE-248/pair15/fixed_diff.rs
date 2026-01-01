                    panic!("panic in unsafe block caught later");
        match handle.join() {
            Ok(val) => val,
            Err(_) => {
                -1
            }
        }
