    fn subtract_unchecked(&mut self, amount: u32) -> u32 {
        unsafe {
            let ptr: *mut u32 = &mut self.value;           
            *ptr = *ptr - amount;                           
            *ptr
        let res = container.subtract_unchecked(amount);
        Ok(res)
    let result = process(shared.clone(), 1);
    match result {
