        let l2 = self.second.lock().unwrap();
        *l1 + *l2
