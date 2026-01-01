    fn compute(&self, input: u64) -> Result<u16, &'static str> {
        let product = input.checked_mul(self.factor).ok_or("Multiplication overflow")?;
        if product > (u16::MAX as u64) {
            return Err("Result exceeds maximum allowed value");
        }
        Ok(product as u16)
fn run(input: u64) -> Result<u16, &'static str> {
            proc_clone.compute(input)
        let res = handle.join().unwrap()?;
        results.push(res);
    Ok(results.into_iter().sum())
    match run(input) {
        Ok(total) => println!("Total: {}", total),
        Err(e) => println!("Error: {}", e),
    }
