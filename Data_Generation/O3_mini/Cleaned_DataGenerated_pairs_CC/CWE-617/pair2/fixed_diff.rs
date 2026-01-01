    fn process(&self, index: usize) -> Result<(), &'static str>;
    fn process(&self, index: usize) -> Result<(), &'static str> {
        if index >= self.elements.len() {
            return Err("Index out-of-range");
        }
        let value = self.elements[index];
        if value <= 100 {
            return Err("Invariant violation: element is not > 100");
        }
        Ok(())
fn run_task(index: usize, cnt: Arc<Container>) -> Result<(), &'static str> {
    let cnt_safe = Arc::clone(&cnt);
    let handle = thread::spawn(move || {
        cnt_safe.process(index)
    handle.join().unwrap()
    let mut user_index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
        1
    let container = Arc::new(Container::create());
    if user_index >= container.elements.len() || container.elements[user_index] <= 100 {
        eprintln!("Provided index is unsafe. Falling back to index = 2.");
        user_index = 2;
    }
    match run_task(user_index, container) {
        Ok(_) => println!("Processing completed successfully."),
        Err(err) => println!("Processing failed: {}", err),
    }
