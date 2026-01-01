    secret_hash: u64, 
        if let Some(record) = records.get(name) {
            if record.secret_hash == Manager::compute(secret) {
                return Ok(());
            } else {
                return Err("Incorrect secret for existing record");
            }
            return Err("Record not found");
