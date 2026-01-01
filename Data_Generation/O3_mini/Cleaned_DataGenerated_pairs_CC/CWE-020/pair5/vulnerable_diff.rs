        let mut buffer = vec![0u8; 10];
            std::ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), 10);
