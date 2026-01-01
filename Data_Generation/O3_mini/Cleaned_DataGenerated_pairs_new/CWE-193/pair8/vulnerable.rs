use std::thread;
struct DataHandler;
impl DataHandler {
    pub fn combine(&self, input: &[u8]) -> Vec<u8> {
        let len = input.len();
        let mut output = Vec::with_capacity(len);
        unsafe {
            std::ptr::copy_nonoverlapping(input.as_ptr(), output.as_mut_ptr(), len);
            output.set_len(len + 1);
        }
        output
    }
}
fn main() {
    let handler = DataHandler;
    let input = b"test";
    let handle = thread::spawn(move || {
        let result = handler.combine(input);
        println!("Combined output: {:?}", result);
    });
    handle.join().unwrap();
}
