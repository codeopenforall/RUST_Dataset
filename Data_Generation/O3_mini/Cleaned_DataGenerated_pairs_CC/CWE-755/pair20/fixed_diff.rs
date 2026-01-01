use std::error::Error;
    fn load() -> Result<Self, Box<dyn Error>> {
        let data = fs::read_to_string("config.txt")?;
        let factor = data.trim().parse::<u32>()?;
        Ok(Settings { factor })
fn process(input: u32) -> Result<u32, Box<dyn Error>> {
    let settings = Settings::load()?; 
        Ok(*ptr.offset(0))
fn execute(input: u32) -> Result<u32, Box<dyn Error>> {
    process(input)
        let computed = process(5).expect("Processing failed");
    handle.join().expect("Thread panicked");
