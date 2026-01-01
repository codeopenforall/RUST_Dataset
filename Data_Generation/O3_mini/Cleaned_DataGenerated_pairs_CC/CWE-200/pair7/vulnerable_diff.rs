    unsafe {
        let secret_ptr: *const u8 = holder.secret.as_ptr();
        let leaked = std::slice::from_raw_parts(secret_ptr, holder.secret.len());
        let leaked_str = std::str::from_utf8_unchecked(leaked);
        if leaked_str.len() > 0 {
            return Err(format!("Operation failed on data: {}", leaked_str));
        }
