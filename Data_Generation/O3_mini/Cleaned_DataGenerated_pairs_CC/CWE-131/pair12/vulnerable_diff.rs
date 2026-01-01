use std::mem;
        let cap = mem::size_of_val(&s) + 1;
        let len = self.cap - 1;
        let slice = std::slice::from_raw_parts(self.raw, len);
            let layout = Layout::from_size_align(self.cap, 1).unwrap();
