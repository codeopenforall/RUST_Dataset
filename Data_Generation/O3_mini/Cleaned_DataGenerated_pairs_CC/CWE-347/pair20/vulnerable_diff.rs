        unsafe {
            let ptr = self.sig.as_ptr() as *const [u8; 64];
            let _sig_arr = *ptr; 
            self.data.len() > 0
        }
         sig: vec![0u8; 32], 
             panic!("Invalid cryptographic check!");
