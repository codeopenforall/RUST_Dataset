        let guard_b = self.lock_b.lock().unwrap();
        let guard_a = self.lock_a.lock().unwrap(); 
        drop(guard_a);
