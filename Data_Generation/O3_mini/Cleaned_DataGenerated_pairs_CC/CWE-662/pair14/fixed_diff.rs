        let guard_a = self.lock_a.lock().unwrap();
        let guard_b = self.lock_b.lock().unwrap();
        drop(guard_a);
