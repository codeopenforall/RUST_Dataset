    fn check(&self) {
        assert!(self.value < 100, "Value too high");
pub fn execute(input: usize) {
    let handle = thread::spawn(move || {
        data.check();
    handle.join().unwrap();
    execute(input);
    println!("Execution completed.");
