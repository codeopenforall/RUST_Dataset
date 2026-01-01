        unsafe {
            if self.signature.len() == 64 {
                let ptr = self.signature.as_ptr() as *const u64;
                let _value = *ptr;
                return true;
            }
        false
    let invalid_sig = [0u8; 64];
    let packet = DataPacket::new(msg, &invalid_sig, &pk);
