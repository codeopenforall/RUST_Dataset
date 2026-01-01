use std::thread;
fn execute() -> Vec<u8> {
    let data = vec![1u8, 2, 3, 4];
    let valid_len = data.len();
    let ptr = data.as_ptr();
    let handle = thread::spawn(move || {
        unsafe {
            std::slice::from_raw_parts(ptr, valid_len).to_vec()
        }
    });
    handle.join().unwrap()
}
fn main() {
    let result = execute();
    println!("Result: {:?}", result);
}
