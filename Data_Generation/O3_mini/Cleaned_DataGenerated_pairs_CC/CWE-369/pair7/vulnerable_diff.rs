    fn compute(&self) -> i32 {
            *num_ptr / *den_ptr
    let den: i32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        let result = calc_clone.compute();
        println!("Computed result: {}", result);
