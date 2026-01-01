        if self.size == 0 {
            return Vec::new();
        }
        let copy_len = self.size - 1; 
        let mut output = vec![0u8; copy_len];
        ptr::copy_nonoverlapping(offset_ptr, output.as_mut_ptr(), copy_len);
