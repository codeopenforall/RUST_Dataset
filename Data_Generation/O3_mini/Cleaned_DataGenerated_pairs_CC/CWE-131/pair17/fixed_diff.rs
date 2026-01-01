            let size = input.len();
            let layout = std::alloc::Layout::from_size_align(self.len, 1).unwrap();
