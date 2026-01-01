        if len > bytes.len() {
            return Err("Input length exceeds available data".into());
        }
            if len_val > bytes.len() {
                return "Error: Length exceeds data".to_string();
            }
        let result = handler.join().map_err(|_| "Thread join error")?;
        if result == "Error: Length exceeds data" {
            return Err("Input length exceeds available data".into());
        }
        Ok(result)
    let user_length = "5";  
