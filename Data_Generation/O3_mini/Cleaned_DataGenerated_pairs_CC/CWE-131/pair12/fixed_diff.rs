        let cap = s.len();
        let slice = std::slice::from_raw_parts(self.raw, self.cap);
            let layout = std::alloc::Layout::from_size_align(self.cap, 1).unwrap();
