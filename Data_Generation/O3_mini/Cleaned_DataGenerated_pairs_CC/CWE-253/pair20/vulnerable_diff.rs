            return Ok(());
            return Err("Unexpected positive value".to_string());
                    Err(_e) => {
                        let mut vec_lock = data_clone.lock().unwrap();
                        vec_lock.push(value);
