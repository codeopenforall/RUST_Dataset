        let product = self.count.checked_mul(self.element_size)
            .expect("Integer multiplication overflow detected");
        product as usize
        let layout = Layout::from_size_align(size, 8).expect("Invalid layout");
        t.join().expect("Thread failed");
