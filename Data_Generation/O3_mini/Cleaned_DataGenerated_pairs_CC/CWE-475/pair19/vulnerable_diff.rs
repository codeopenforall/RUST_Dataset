        let mut output = vec![0u8; self.size];
        ptr::copy_nonoverlapping(offset_ptr, output.as_mut_ptr(), self.size);
