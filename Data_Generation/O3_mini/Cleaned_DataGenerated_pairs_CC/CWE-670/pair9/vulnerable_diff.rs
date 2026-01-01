        unsafe {
            let func_ptr = *self.table.get_unchecked(index as usize);
            func_ptr(input)
