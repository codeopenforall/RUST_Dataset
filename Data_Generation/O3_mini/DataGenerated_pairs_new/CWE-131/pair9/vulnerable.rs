//////////////////////////////
// Vulnerable Code Example
//////////////////////////////

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
    // This function packs the Packet into a byte buffer.
    // It incorrectly calculates the buffer length for the string by using size_of_val on the String struct,
    // which yields the size of the String's internal structure rather than the actual byte length of its content.
    pub unsafe fn pack(&self) -> Vec<u8> {
        // Determine the number of bytes for the header (u32)
        let header_size = mem::size_of_val(&self.tag);
        // Vulnerability: Incorrectly using size_of_val on the String, rather than using the actual content length.
        let content_size = mem::size_of_val(&self.content); 
        let total_size = header_size + content_size;

        let mut buffer = Vec::with_capacity(total_size);
        buffer.set_len(total_size);

        // Copy header into buffer
        ptr::copy_nonoverlapping(
            &self.tag as *const u32 as *const u8,
            buffer.as_mut_ptr(),
            header_size,
        );
        // Copy string content into buffer.
        // NOTE: This copies memory from the string's internal pointer.
        // Due to the misuse of size, it attempts to copy too many bytes.
        ptr::copy_nonoverlapping(
            self.content.as_ptr(),
            buffer.as_mut_ptr().add(header_size),
            content_size,
        );
        buffer
    }
}

fn main() {
    // Create a Packet with a small content.
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
                // The printed buffer will have an incorrect size due to the vulnerability.
                println!("Packed bytes: {:?}", buf);
            }
        }));
    }
    for h in handles {
        h.join().expect("Thread panicked");
    }
}