struct Handler {
impl Handler {
        Handler { sender }
    fn run(&self) {
        let sender_copy = self.sender.clone();
            let mut buffer = Vec::with_capacity(SIZE);
                buffer.set_len(SIZE);
            sender_copy.send(buffer).unwrap();
fn execute_jobs(count: usize) -> () {
    let (tx, rx) = channel();
    let handler = Handler::new(tx);
    for _ in 0..count {
        handler.run();
    for _ in 0..count {
    execute_jobs(jobs);
    Ok(())
    let arguments: Vec<String> = env::args().collect();
    let num_jobs: usize = if arguments.len() > 1 {
        arguments[1].parse().unwrap_or(0)
    run_app(num_jobs).unwrap();
    println!("Processing complete");
