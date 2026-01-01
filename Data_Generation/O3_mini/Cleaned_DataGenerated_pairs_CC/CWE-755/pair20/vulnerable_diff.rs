    fn load() -> Self {
        let data = fs::read_to_string("config.txt").unwrap_or_default();
        let factor = data.trim().parse::<u32>().unwrap_or_default();
        Settings { factor }
fn process(input: u32) -> u32 {
    let settings = Settings::load();
        *ptr.offset(0)
fn execute(input: u32) -> Result<u32, &'static str> {
    let result = process(input);
    Ok(result)
        let computed = process(5);
    handle.join().unwrap();
