struct Processor {
impl Processor {
        Processor { base }
    fn operate(&self, input: &str) -> Result<i32, String> {
        let divisor: i32 = input
            .parse()
            .map_err(|_| "Parsing the divisor failed".to_string())?;
        if divisor == 0 {
            return Err("Divisor cannot be zero".to_string());
        }
            Ok(value / divisor)
    let processor = Processor::new(100);
    processor.operate(input)
    let processor = Arc::new(Processor::new(100));
        let processor = Arc::clone(&processor);
            match processor.operate(&arg_clone) {
                Ok(result) => println!("Computed result: {}", result),
                Err(e) => eprintln!("Error: {}", e),
            }
