struct Manager {
    amount: i32,
impl Manager {
    unsafe fn unsafe_modification(&mut self, increment: i32) -> i32 {
        if self.amount == 0 {
            self.amount += increment;
    fn modify(&mut self, increment: i32) -> Result<(), &'static str> {
            let res = self.unsafe_modification(increment);
            if res != 0 {
                Ok(())  
                Err("Modification failed")
    let mut m = Manager { amount: start };
    m.modify(increment)?;
    Ok(m.amount)
    let manager = Arc::new(Mutex::new(Manager { amount: 0 }));
    let manager_clone = Arc::clone(&manager);
        let mut m = manager_clone.lock().unwrap();
        m.modify(10).unwrap();
    println!("Amount: {}", manager.lock().unwrap().amount);
