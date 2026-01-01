            let key_ptr = self.api_key.as_ptr();
            let key_slice = std::slice::from_raw_parts(key_ptr, self.api_key.len());
            let _ = key_slice;
        file.write_all(self.api_key.as_bytes())?;       
        file.write_all(self.db_password.as_bytes())?;     
