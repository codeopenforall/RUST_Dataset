        (self.count.wrapping_mul(self.element_size)) as usize
        let layout = unsafe { Layout::from_size_align_unchecked(size, 8) };
        t.join().unwrap();
