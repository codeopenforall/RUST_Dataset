    pub unsafe fn traverse(&self, data: *const u8, len: usize, idx: usize, depth: usize, max_depth: usize) -> Result<i32, &'static str> {
        if depth > max_depth {
            return Err("Maximum recursion depth exceeded");
        }
            return Ok(0);
            let next = self.traverse(data, len, idx + 1, depth + 1, max_depth)?;
            Ok(1 + next)
            Ok(0)
    pub fn process(&self, input: &[u8], max_depth: usize) -> Result<i32, &'static str> {
        unsafe { self.traverse(input.as_ptr(), input.len(), 0, 0, max_depth) }
    let inp = input.as_bytes().to_vec();
    let max_depth = 1000;
        engine.process(&inp, max_depth)
        Ok(res) => res,
        Err(_) => Err("Thread panicked unexpectedly"),
        Ok(res) => println!("Processing successful, result: {}", res),
        Err(err) => println!("Processing failed with error: {}", err),
