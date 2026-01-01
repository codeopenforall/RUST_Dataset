        let size = input.len();
        ptr::copy_nonoverlapping(input.as_ptr(), buf, size);
            let layout = std::alloc::Layout::from_size_align(self.len, 1).unwrap();
