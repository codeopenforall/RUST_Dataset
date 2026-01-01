    fn dangerous_sum(&self, offset: usize, count: usize) -> i32 {
            slice.iter().sum()
        let res = panic::catch_unwind(|| self.dangerous_sum(offset, count));
        match res {
            Ok(val) => Ok(val),
            Err(_) => Err("Operation panicked"),
        }
