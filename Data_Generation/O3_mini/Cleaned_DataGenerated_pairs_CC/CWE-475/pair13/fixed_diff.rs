        let mut buf = Vec::with_capacity(data.len());
            ptr::copy_nonoverlapping(data.as_ptr(), buf.as_mut_ptr(), data.len());
            buf.set_len(data.len());
