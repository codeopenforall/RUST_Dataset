use std::slice;

trait Processor {
    fn run(&self, data: &[u8]) -> u32;
}

struct Calculator;
impl Processor for Calculator {
    fn run(&self, data: &[u8]) -> u32 {
        // This implementation violates the API precondition:
        // It blindly offsets the pointer by 1 and computes the slice length as data.len()-1.
        // When data is empty, data.len()-1 underflows to a very large number,
        // and offset(1) produces an invalid pointer. This undefined behavior (CWE-475)
        // is triggered by providing an empty input.
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
    // Print the computed sum from bytes[1..].
    let res = calc.run(bytes);
    println!("{}", res);
}