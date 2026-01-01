        if len < 2 {
            return 0;
        }
        for i in 0..(len - 1) {
            let first = self.data[i];
            let second = self.data[i + 1];
            total = total.wrapping_add(first).wrapping_add(second);
