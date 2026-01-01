use std::ptr;
    pub unsafe fn update(&mut self, idx: usize, val: u32) -> Result<(), &'static str> {
        let ptr = self.data.as_mut_ptr();
        *ptr.add(idx) = val; 
    unsafe { buf.update(10, 100) }?;
