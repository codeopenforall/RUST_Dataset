    unsafe fn subtract(&mut self, sub: u64) {
        let ptr = &mut self.value as *mut u64;
        *ptr = *ptr - sub; 
    unsafe {
        comp.subtract(sub);
    }
