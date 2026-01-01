//////////////////////////////
// Vulnerable Code Snippet
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
/// Vulnerability: The buffer size is computed as the size of the u32 header plus
/// the size of the String value obtained via mem::size_of_val, which returns the size
/// of the String struct (i.e. pointer, length, capacity) rather than the actual
/// length of the UTF-8 encoded message. When the message is longer than this fixed size,
/// the copy of the message bytes gets truncated.
fn serialize(record: &Record) -> Vec<u8> {
    unsafe {
        let header_size = mem::size_of::<u32>();
        // Incorrect: using the size of the String struct instead of the dynamic text length.
        let msg_struct_size = mem::size_of_val(&record.message);
        let total_size = header_size + msg_struct_size;
        let layout = Layout::from_size_align(total_size, 1).unwrap();
        let buffer = alloc(layout);
        if buffer.is_null() {
            panic!("Memory allocation failed");
        }
        // Copy the id into the buffer.
        ptr::copy_nonoverlapping(
            &record.id as *const u32 as *const u8,
            buffer,
            header_size,
        );
        // Incorrectly copies only 'msg_struct_size' bytes from the actual message bytes.
        ptr::copy_nonoverlapping(
            record.message.as_ptr(),
            buffer.add(header_size),
            msg_struct_size,
        );
        Vec::from_raw_parts(buffer, total_size, total_size)
    }
}

fn main() {
    // Create a test record with a message longer than the stored size of a String struct.
    // On most systems, mem::size_of::<String>() yields 24, so a message of 50 bytes 
    // will trigger the truncation.
    let rec = Record::new(42, "A".repeat(50));
    let buf = serialize(&rec);
    println!("Serialized buffer length: {}", buf.len());
    // Note: The program does not verify serialization correctness.
}