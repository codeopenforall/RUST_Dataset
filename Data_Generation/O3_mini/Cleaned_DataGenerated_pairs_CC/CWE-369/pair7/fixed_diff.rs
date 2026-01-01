    fn compute(&self) -> Option<i32> {
        if self.denominator == 0 {
            return None;
        }
            Some(*num_ptr / *den_ptr)
    let den: i32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
        match calc_clone.compute() {
            Some(result) => println!("Computed result: {}", result),
            None => println!("Error: Denominator is zero."),
        }
