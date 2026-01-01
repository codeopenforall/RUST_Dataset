use std::slice;

trait Processor {
    fn run(&self, data: &[u8]) -> u32;
}

struct Calculator;
impl Processor for Calculator {
    fn run(&self, data: &[u8]) -> u32 {
        // Validate the precondition: input must be nonempty.
        if data.is_empty() {
            // Instead of trying to process an empty slice unsafely,
            // return a defined value (here 0) which is the correct behavior.
            return 0;
        }
        unsafe {
            let ptr = data.as_ptr().offset(1);
            let sub_slice = slice::from_raw_parts(ptr, data.len() - 1);
            sub_slice.iter().fold(0, |acc, &x| acc + x as u32)
        }
    }
}

fn main() {
    // Accept an input string from command-line argument.
    let input = std::env::args().nth(1).unwrap_or_default();
    let bytes = input.as_bytes();
    let calc = Calculator;
    // Print the computed sum from bytes[1..] or 0 if input is empty.
    let res = calc.run(bytes);
    println!("{}", res);
}