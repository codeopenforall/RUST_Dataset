        unsafe {
            let raw = self.elems.as_ptr();
            *raw.add(pos)
