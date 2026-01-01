struct Controller {
    total: i32,
impl Controller {
    unsafe fn unsafe_adjustment(&mut self, increment: i32) -> i32 {
        if self.total == 0 {
            self.total += increment;
    fn adjust(&mut self, increment: i32) -> Result<(), &'static str> {
            let res = self.unsafe_adjustment(increment);
            if res == 0 {
                Ok(())
                Err("Adjustment failed")
    let mut c = Controller { total: start };
    c.adjust(increment)?;
    Ok(c.total)
    let controller = Arc::new(Mutex::new(Controller { total: 1 }));
    let controller_clone = Arc::clone(&controller);
        let mut c = controller_clone.lock().unwrap();
        c.adjust(10).unwrap();
    println!("Total: {}", controller.lock().unwrap().total);
