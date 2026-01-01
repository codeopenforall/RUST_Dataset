use std::thread;
struct Processor;
impl Processor {
    fn operate(&self, input: &[u32]) -> Option<u32> {
        if input.len() < 1 {
            return None;
        }
        unsafe {
            let mut buf = [0u32; 1];
            std::ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), 1);
            Some(buf[0])
        }
    }
}
fn main() {
    let proc_inst = Processor;
    let data: Vec<u32> = vec![42];
    let handle = thread::spawn(move || {
        proc_inst.operate(&data)
    });
    match handle.join().unwrap() {
        Some(val) => println!("Output: {}", val),
        None => println!("No output produced"),
    }
}
