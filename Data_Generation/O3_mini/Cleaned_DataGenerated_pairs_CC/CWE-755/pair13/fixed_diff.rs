struct Processor {
impl Processor {
    fn new(input: &str) -> Result<Self, String> {
        let parsed = input.parse::<u64>().map_err(|e| format!("Parsing error: {}", e))?;
        Ok(Processor { value: parsed })
    fn add_one(&mut self) {
    let mut proc = Processor::new(input)?;
    proc.add_one();
    Ok(proc.value)
    let outputs = Arc::new(Mutex::new(Vec::new()));
    let errors = Arc::new(Mutex::new(Vec::<String>::new()));
        let out_clone = Arc::clone(&outputs);
        let err_clone = Arc::clone(&errors);
            match process_input(inp) {
                Ok(val) => {
                    let mut o = out_clone.lock().unwrap();
                    o.push(val);
                }
                Err(e) => {
                    let mut er = err_clone.lock().unwrap();
                    er.push(e);
                }
            }
    for th in threads {
        th.join().unwrap();
    let collected_errors = errors.lock().unwrap();
    if !collected_errors.is_empty() {
        println!("Error occurred during processing: {:?}", *collected_errors);
        std::process::exit(1);
    } else {
        let out = outputs.lock().unwrap();
        let sum: u64 = out.iter().sum();
        println!("Sum: {}", sum);
    }
