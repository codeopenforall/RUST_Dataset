            let size = mem::size_of_val(&input);
            let layout = Layout::from_size_align(self.len, 1).unwrap();
