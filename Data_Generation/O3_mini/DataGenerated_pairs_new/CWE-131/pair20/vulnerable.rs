///////////////////////////////////////
// Vulnerability Example Code Start
///////////////////////////////////////
use std::mem::size_of;
use std::ptr;
use std::thread;

/// Processes an input string by copying its bytes into a buffer using an unsafe operation.
/// In this variant, the buffer size is calculated incorrectly using the size of the String struct,
/// not the actual length of its content.
fn process() -> String {
    let original = String::from("This is a test message that exceeds the typical struct size.");
    let output = thread::spawn(move || {
        unsafe {
            // Incorrect calculation: Using size_of on the String type returns the size of the struct,
            // not the length of its underlying data.
            let copy_len = size_of::<String>(); // typically 24 bytes on 64-bit systems
            // Allocate a buffer with insufficient capacity to hold the entire string data.
            let mut buffer: Vec<u8> = Vec::with_capacity(copy_len);
            buffer.set_len(copy_len);
            // Unsafe memory copy: Copies only copy_len bytes from the original string.
            ptr::copy_nonoverlapping(original.as_ptr(), buffer.as_mut_ptr(), copy_len);
            // Convert the copied bytes back to a UTF-8 string; may be truncated.
            String::from_utf8_lossy(&buffer).into_owned()
        }
    }).join().unwrap();
    output
}

fn main() {
    // The main function executes the process. The printed output may be incomplete due
    // to the buffer size miscalculation.
    let result = process();
    println!("{}", result);
}
///////////////////////////////////////
// Vulnerability Example Code End
///////////////////////////////////////