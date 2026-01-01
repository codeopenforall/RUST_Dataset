const MAX_TOTAL: usize = 50 * 1024 * 1024; 
struct Processor {
impl Processor {
        Processor { sender }
    fn dispatch(&self) {
        let sender_clone = self.sender.clone();
            let mut data = Vec::with_capacity(SIZE);
                data.set_len(SIZE);
            sender_clone.send(data).unwrap();
fn process_jobs(job_count: usize) -> Result<(), &'static str> {
    if job_count * SIZE > MAX_TOTAL {
        return Err("Requested resource exceeds safe limits");
    let (tx, rx) = channel();
    let processor = Processor::new(tx);
    for _ in 0..job_count {
        processor.dispatch();
    }
    for _ in 0..job_count {
    Ok(())
    process_jobs(jobs)
    let args: Vec<String> = env::args().collect();
    let num_jobs: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    match run_app(num_jobs) {
        Ok(_) => println!("Processing complete"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
