    unsafe fn process(&self, index: usize);
    unsafe fn process(&self, index: usize) {
        let ptr = self.elements.as_ptr();
        let value = *ptr.add(index);
        assert!(value > 100, "Invariant failure: element is not > 100");
fn run_task(index: usize, cnt: Arc<Container>) {
    let handle = thread::spawn(move || unsafe {
        cnt.process(index);
    handle.join().unwrap();
    let user_index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
        0
    let shared = Arc::new(Container::create());
    run_task(user_index, shared);
