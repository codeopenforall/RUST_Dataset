    fn compute(&self, start: usize, count: usize) -> i32;
    fn compute(&self, start: usize, count: usize) -> i32 {
        let mut total = 0;
        unsafe {
            for i in 0..count {
                total += *slice.get_unchecked(start + i);
            }
        total
    let result = proc_inst.compute(3, 2);
    println!("Computed sum: {}", result);
