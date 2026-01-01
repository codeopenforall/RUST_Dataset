struct DataProcessor;
impl DataProcessor {
    pub fn execute(&self, requested: usize) -> Result<u8, &'static str> {
    let processor = DataProcessor;
    processor.execute(requested)
    match process_input(20) {
        Ok(val) => println!("Processed value: {}", val),
