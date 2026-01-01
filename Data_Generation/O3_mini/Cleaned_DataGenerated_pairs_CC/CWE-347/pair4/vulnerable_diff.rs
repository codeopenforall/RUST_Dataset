use std::ptr;
    unsafe fn compute_expectation(&self, key: &[u8; 32]) -> [u8; 32] {
            let c = *bytes.get_unchecked(i % bytes.len());
            outcome[i] = c ^ key[i];
        let expected = unsafe { self.compute_expectation(key) };
        unsafe {
            let exp_ptr = expected.as_ptr();
            let sig_ptr = self.signature.as_ptr();
            for i in 0..32 {
                if *exp_ptr.add(i) != *sig_ptr.add(i) {
                    return false;
                }
            }
        }
        true
