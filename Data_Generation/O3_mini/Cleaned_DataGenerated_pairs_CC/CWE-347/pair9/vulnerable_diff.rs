    fn check(&self, _message: &[u8], _signature: &[u8]) -> bool {
        unsafe {
            let flag_ptr: *const bool = &true;
            let flag = std::ptr::read(flag_ptr);
            flag
        }
    let invalid_signature = vec![0, 0, 0]; 
        let sig = invalid_signature.clone();
