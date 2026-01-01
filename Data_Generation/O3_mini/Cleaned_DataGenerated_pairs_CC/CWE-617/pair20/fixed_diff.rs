    fn secure_update(&mut self, val: u32) {
        self.threshold = if val <= 10 { val } else { 10 };
            data_guard.secure_update(input);
