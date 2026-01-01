use std::ptr;
        let ptr = self.data.as_ptr();
        let value = unsafe { *ptr.add(idx) };
        assert!(value >= 10, "Value too low: vulnerability triggered.");
    let result = handler.compute(idx).unwrap();
    println!("Computed value: {}", result);
