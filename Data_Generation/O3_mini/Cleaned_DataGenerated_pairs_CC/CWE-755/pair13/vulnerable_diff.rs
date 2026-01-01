struct Handler {
impl Handler {
    fn new(input: &str) -> Self {
        let parsed = input.parse::<u64>().unwrap_or_default(); 
        Handler { value: parsed }
    fn increment(&mut self) {
    let mut handler = Handler::new(input);
    handler.increment();
    Ok(handler.value)
    let results = Arc::new(Mutex::new(Vec::new()));
        let res_clone = Arc::clone(&results);
            let result = process_input(inp).unwrap();
            let mut guard = res_clone.lock().unwrap();
            guard.push(result);
    for thread in threads {
        thread.join().unwrap();
    let guard = results.lock().unwrap();
    let sum: u64 = guard.iter().sum();
    println!("Sum: {}", sum);
