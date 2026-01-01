        if self.counter.checked_add(inc).map_or(true, |tmp| tmp >= 100) {
            println!("Update rejected: operation would exceed threshold.");
            return self.counter;
        self.counter += inc;
