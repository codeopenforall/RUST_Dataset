        unsafe {
            let value = *self.data.get_unchecked(idx);
            Ok(value)
