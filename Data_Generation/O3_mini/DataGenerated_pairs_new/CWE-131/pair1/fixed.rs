//////////////////////////////
// Corrected Code Snippet
//////////////////////////////
use std::alloc::{alloc, Layout};
use std::mem;
use std::ptr;

#[derive(Debug)]
struct Record {
    id: u32,
    message: String,
}

impl Record {
    fn new(id: u32, message: String) -> Self {
        Record { id, message }
    }
}

/// Serializes the record into a contiguous byte buffer.
/// 
/// Correction: The buffer size now correctly uses the actual message length (in bytes)
/// rather than the size of the String struct. This ensures the entire UTF-8 encoded text
/// is copied.
fn serialize(record: &Record) -> Vec<u8> {
    unsafe {
        let header_size = mem::size_of::<u32>();
        // Correct: use the dynamic byte length of the message.
        let msg_data_len = record.message.len();
        let total_size = header_size + msg_data_len;
        let layout = Layout::from_size_align(total_size, 1).unwrap();
        let buffer = alloc(layout);
        if buffer.is_null() {
            panic!("Memory allocation failed");
        }
        // Copy the id.
        ptr::copy_nonoverlapping(
            &record.id as *const u32 as *const u8,
            buffer,
            header_size,
        );
        // Copy the complete message.
        ptr::copy_nonoverlapping(
            record.message.as_ptr(),
            buffer.add(header_size),
            msg_data_len,
        );
        Vec::from_raw_parts(buffer, total_size, total_size)
    }
}

fn main() {
    // Create a test record with a message longer than the fixed-size of a String struct.
    let rec = Record::new(42, "A".repeat(50));
    let buf = serialize(&rec);
    println!("Serialized buffer length: {}", buf.len());
    // The printed length should correctly be 4 (u32) + 50 (message) = 54 bytes.
}