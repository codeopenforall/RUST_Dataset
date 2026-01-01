    fn increment(&mut self) {
        self.count = self.count.wrapping_add(1);
        let mut guard = self.data.write().unwrap();
    let ctl = Controller::new();
    let shared_ctrl = Arc::new(ctl);
