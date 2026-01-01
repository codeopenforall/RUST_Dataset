    unsafe fn release(&mut self);
    is_freed: bool,
            is_freed: false,
    pub unsafe fn process(&mut self) {
    unsafe fn release(&mut self) {
        if self.is_freed {
            return; 
        self.is_freed = true;
            if !self.is_freed {
                let _ = Box::from_raw(self.ptr);
                self.is_freed = true;
        let mut resource = Manager::initialize(100);
