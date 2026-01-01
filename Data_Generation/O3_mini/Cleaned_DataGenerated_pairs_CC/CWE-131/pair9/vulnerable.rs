#![allow(dead_code)]
use std::mem;
use std::ptr;
use std::sync::Arc;
use std::thread;
struct Packet {
    tag: u32,
    content: String,
}
impl Packet {
    pub unsafe fn pack(&self) -> Vec<u8> {
        let header_size = mem::size_of_val(&self.tag);
        let content_size = mem::size_of_val(&self.content); 
        let total_size = header_size + content_size;
        let mut buffer = Vec::with_capacity(total_size);
        buffer.set_len(total_size);
        ptr::copy_nonoverlapping(
            &self.tag as *const u32 as *const u8,
            buffer.as_mut_ptr(),
            header_size,
        );
        ptr::copy_nonoverlapping(
            self.content.as_ptr(),
            buffer.as_mut_ptr().add(header_size),
            content_size,
        );
        buffer
    }
}
fn main() {
    let pkt = Arc::new(Packet {
        tag: 0xDEADBEEF,
        content: String::from("Hello"),
    });
    let mut handles = vec![];
    for _ in 0..4 {
        let pkt_clone = pkt.clone();
        handles.push(thread::spawn(move || {
            unsafe {
                let buf = pkt_clone.pack();
                println!("Packed bytes: {:?}", buf);
            }
        }));
    }
    for h in handles {
        h.join().expect("Thread panicked");
    }
}
