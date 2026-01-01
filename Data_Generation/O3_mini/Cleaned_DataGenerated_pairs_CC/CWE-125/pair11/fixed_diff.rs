    fn compute(&self, start: usize, count: usize) -> Result<i32, &'static str>;
    fn compute(&self, start: usize, count: usize) -> Result<i32, &'static str> {
        if start.checked_add(count).map_or(true, |end| end > slice.len()) {
            return Err("Index out-of-bounds");
        let mut total = 0;
        for i in 0..count {
            total += slice.get(start + i).unwrap();
        }
        Ok(total)
    match proc_inst.compute(3, 2) {
        Ok(result) => println!("Computed sum: {}", result),
        Err(e) => println!("Error: {}", e),
    }
