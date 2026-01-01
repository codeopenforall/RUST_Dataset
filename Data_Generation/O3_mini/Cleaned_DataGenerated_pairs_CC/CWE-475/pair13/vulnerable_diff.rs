        let mut buf: Vec<u8> = Vec::with_capacity(10);
        let copy_len = data.len() + 2;
            ptr::copy_nonoverlapping(data.as_ptr(), buf.as_mut_ptr(), copy_len);
            buf.set_len(copy_len);
