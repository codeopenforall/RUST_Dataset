//////////////////////////////
// Corrected Code Example
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
    // The fix correctly calculates the buffer length for the string using its actual content length.
    pub unsafe fn pack(&self) -> Vec<u8> {
        // Determine the number of bytes for the header (u32)
        let header_size = mem::size_of_val(&self.tag);
        // Correctly compute the content size by using the length of the string in bytes.
        let content_size = self.content.len(); 
        let total_size = header_size + content_size;

        let mut buffer = Vec::with_capacity(total_size);
        buffer.set_len(total_size);

        // Copy header into buffer
        ptr::copy_nonoverlapping(
            &self.tag as *const u32 as *const u8,
            buffer.as_mut_ptr(),
            header_size,
        );
        // Copy only the actual string content into the buffer.
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
                // The printed buffer will have the correct size (header + 5 bytes for "Hello").
                println!("Packed bytes: {:?}", buf);
            }
        }));
    }
    for h in handles {
        h.join().expect("Thread panicked");
    }
}