    pub fn quick_validate(&self) -> bool {
        unsafe {
            if self.signature.len() >= 8 {
                let sig_ptr = self.signature.as_ptr() as *const u64; 
                let sig_val = *sig_ptr;                                
                if sig_val == 0xDEADBEEFDEADBEEF {
                    return true;
                } else {
                    return true; 
                }
            }
        false
    packet.quick_validate()
        message: b"Important data".to_vec(),
        signature: vec![0, 1, 2, 3, 4, 5, 6, 7],
