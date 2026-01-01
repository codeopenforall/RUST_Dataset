use std::env;
static DATA: &[u8] = b"Example static data used for unsafe string extraction vulnerability demo";
struct Processor;
impl Processor {
    fn process(&self, offset: usize, len: usize) -> Result<String, String> {
        if offset.checked_add(len).map_or(true, |end| end > DATA.len()) {
            return Err("Invalid input: offset and length exceed data bounds".to_string());
        }
        let slice = &DATA[offset..offset+len];
        let s = std::str::from_utf8(slice).map_err(|_| "UTF-8 conversion error".to_string())?;
        Ok(s.to_string())
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} offset:len", args[0]);
        return;
    }
    let parts: Vec<&str> = args[1].split(':').collect();
    if parts.len() != 2 {
        println!("Invalid input format, expected offset:len");
        return;
    }
    let offset = parts[0].parse::<usize>().unwrap_or(0);
    let len = parts[1].parse::<usize>().unwrap_or(0);
    let proc_inst = Processor;
    match proc_inst.process(offset, len) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
