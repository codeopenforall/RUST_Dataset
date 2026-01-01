use std::panic;
    fn compute(&self, numerator: i32, denominator: i32) -> i32 {
        unsafe {
            numerator / denominator  
        let res = panic::catch_unwind(|| self.compute(numerator, denominator));
        match res {
            Ok(val) => Ok(val),
            Err(_) => Err(String::from("Panic occurred")),
        }
    let result = handle.join().unwrap();
    println!("Computed result: {}", result);
