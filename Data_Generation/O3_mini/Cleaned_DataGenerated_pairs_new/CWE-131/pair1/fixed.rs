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
fn serialize(record: &Record) -> Vec<u8> {
    unsafe {
        let header_size = mem::size_of::<u32>();
        let msg_data_len = record.message.len();
        let total_size = header_size + msg_data_len;
        let layout = Layout::from_size_align(total_size, 1).unwrap();
        let buffer = alloc(layout);
        if buffer.is_null() {
            panic!("Memory allocation failed");
        }
        ptr::copy_nonoverlapping(
            &record.id as *const u32 as *const u8,
            buffer,
            header_size,
        );
        ptr::copy_nonoverlapping(
            record.message.as_ptr(),
            buffer.add(header_size),
            msg_data_len,
        );
        Vec::from_raw_parts(buffer, total_size, total_size)
    }
}
fn main() {
    let rec = Record::new(42, "A".repeat(50));
    let buf = serialize(&rec);
    println!("Serialized buffer length: {}", buf.len());
}
