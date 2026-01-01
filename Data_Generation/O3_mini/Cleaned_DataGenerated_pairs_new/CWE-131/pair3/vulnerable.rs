use std::mem;
use std::ptr;
use std::thread;
struct Communicator;
impl Communicator {
    fn transmit(&self, input: &str) -> Box<[u8]> {
        let buffer_size = mem::size_of::<&str>(); 
        let mut buffer = vec![0u8; buffer_size].into_boxed_slice();
        unsafe {
            ptr::copy_nonoverlapping(input.as_ptr(), buffer.as_mut_ptr(), input.len());
        }
        buffer
    }
}
fn main() {
    let comm = Communicator;
    let handle = thread::spawn(move || {
        let data = "A".repeat(50);
        let buf = comm.transmit(&data);
        buf.iter().fold(0u8, |acc, &x| acc.wrapping_add(x))
    });
    let checksum = handle.join().expect("Thread panic");
    println!("Resulting checksum: {}", checksum);
}
