use std::panic;
    pub unsafe fn explore(&self, data: *const u8, len: usize, idx: usize) -> i32 {
            return 0;
            return 1 + self.explore(data, len, idx + 1);
            return 0;
    pub fn journey(&self, input: &[u8]) -> i32 {
        unsafe { self.explore(input.as_ptr(), input.len(), 0) }
    let inp = input.as_bytes().to_vec(); 
        engine.journey(&inp)
        Ok(res) => Ok(res),
        Err(_) => Err("Unexpected recursive overflow occurred"),
        Ok(res) => println!("Computation successful, result: {}", res),
        Err(err) => println!("Computation failed with error: {}", err),
