    fn add(&mut self, amount: u32) -> Result<(), &'static str> {
        self.balance = self.balance.checked_add(amount).ok_or("Overflow occurred")?;
        Ok(())
fn run_calculation(init: u32, add: u32, threads: usize) -> Result<u32, &'static str> {
        handles.push(thread::spawn(move || -> Result<(), &'static str> {
            guard.add(add)
        let res = handle.join().map_err(|_| "Thread panicked")?;
        if let Err(e) = res {
            return Err(e);
        }
    Ok(guard.current())
    run_calculation(u32::MAX - 10, 15, 1)
    match try_compute() {
        Ok(val) => println!("Final result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
