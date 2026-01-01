        let buf_len = 10;
        if idx.checked_add(buf_len).map_or(true, |end| end > self.data.len()) {
            return Err("index out of bounds");
        }
        let mut buffer = vec![0u8; buf_len];
            std::ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), buf_len);
