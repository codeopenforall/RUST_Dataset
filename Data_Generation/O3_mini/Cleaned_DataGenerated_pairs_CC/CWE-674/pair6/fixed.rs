use std::env;
const MAX_DEPTH: usize = 1000;
struct Parser<'a> {
    buffer: &'a [u8],
}
impl<'a> Parser<'a> {
    unsafe fn get_byte(&self, index: usize) -> u8 {
        *self.buffer.get_unchecked(index)
    }
    fn parse_recursively(&self, pos: usize, depth: usize) -> Result<(usize, u32), &'static str> {
        if depth > MAX_DEPTH {
            return Err("Recursion limit exceeded");
        }
        let mut index = pos;
        let mut count: u32 = 0;
        while index < self.buffer.len() {
            unsafe {
                match self.get_byte(index) {
                    b'(' => {
                        let (next_index, inner_count) = self.parse_recursively(index + 1, depth + 1)?;
                        count = count.wrapping_add(inner_count).wrapping_add(1);
                        index = next_index;
                    }
                    b')' => {
                        return Ok((index + 1, count));
                    }
                    _ => {
                        index += 1;
                    }
                }
            }
        }
        Ok((index, count))
    }
    fn run(&self) -> Result<u32, &'static str> {
        let (_, result) = self.parse_recursively(0, 0)?;
        Ok(result)
    }
}
fn process(input: &str) -> Result<u32, &'static str> {
    let parser = Parser { buffer: input.as_bytes() };
    parser.run()
}
fn main() {
    let left_parens = "(".repeat(1100);
    let right_parens = ")".repeat(1100);
    let input = left_parens + &right_parens;
    match process(&input) {
        Ok(result) => println!("Parsed count: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
