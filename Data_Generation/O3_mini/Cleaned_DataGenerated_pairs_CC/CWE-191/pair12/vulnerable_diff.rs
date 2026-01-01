    pub unsafe fn reduce(&mut self, sub: u32) -> u32 {
        self.count = self.count - sub;  
        self.count
    let res = unsafe { data.reduce(10) }; 
    Ok(res)
                unsafe {
                    let r = lock.reduce(10);
                    println!("Subtraction result: {}", r);
