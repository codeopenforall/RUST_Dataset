use std::ptr;
        unsafe {
            let mut vec = Vec::with_capacity(count);
            vec.set_len(count);
            for i in 0..count {
                *vec.get_unchecked_mut(i) = 0xAA;
            }
            Ok(vec)
        }
