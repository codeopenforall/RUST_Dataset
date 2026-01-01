    fn bump(&mut self);
    fn bump(&mut self) {
        self.value = self.value.wrapping_add(1);
        let mut guard = shared.lock().unwrap();
        guard.bump(); 
