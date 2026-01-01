use std::ptr;
struct DataHandler;
impl DataHandler {
    pub fn process(&self, input: &[u8]) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(input.len());
        unsafe {
            let dest = buffer.as_mut_ptr();
            for i in 0..input.len() {
                let value = *input.get_unchecked(i);
                ptr::write(dest.add(i), value);
            }
            buffer.set_len(input.len());
        }
        buffer
    }
}
fn main() {
    let handler = DataHandler;
    let sample = vec![10, 20, 30, 40];
    let result = handler.process(&sample);
    println!("Processed output: {:?}", result);
}
