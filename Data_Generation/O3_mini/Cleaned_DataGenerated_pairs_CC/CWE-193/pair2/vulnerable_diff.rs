        unsafe { data.set_len(size); } 
            data[i] = i as u8;
        unsafe {
            *self.data.get_unchecked(self.data.len())
        }
