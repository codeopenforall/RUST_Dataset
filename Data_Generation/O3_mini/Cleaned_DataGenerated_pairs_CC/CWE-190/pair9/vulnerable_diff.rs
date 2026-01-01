    fn add(&mut self, amount: u32) {
        unsafe {
            self.balance = std::intrinsics::unchecked_add(self.balance, amount);
        }
fn run_calculation(init: u32, add: u32, threads: usize) -> u32 {
        handles.push(thread::spawn(move || {
            guard.add(add);
        handle.join().unwrap();
    guard.current()
    let result = run_calculation(u32::MAX - 10, 15, 1);
    Ok(result)
    let result = try_compute().unwrap();
    println!("Final result: {}", result);
