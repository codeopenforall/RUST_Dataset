        unsafe {
            let cand_ptr = candidate.as_ptr();
            let trusted_ptr = trusted.as_ptr();
            if cand_ptr == trusted_ptr {
                return true;
            }
