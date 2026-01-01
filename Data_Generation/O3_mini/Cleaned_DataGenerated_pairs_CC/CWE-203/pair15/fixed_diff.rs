    secret_hash: u64,
        let valid = if let Some(record) = records.get(name) {
            record.secret_hash == Manager::compute(secret)
            false
        };
        if valid {
            Ok(())
        } else {
            Err("Invalid credentials")
