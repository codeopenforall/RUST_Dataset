///////////////////////////////////////
// Corrected Example Code Start
///////////////////////////////////////
use std::ptr;
use std::thread;

/// Processes an input string by copying its bytes into a buffer safely.
/// This corrected variant calculates the proper buffer size using the length of the string.
fn process() -> String {
    let original = String::from("This is a test message that exceeds the typical struct size.");
    let output = thread::spawn(move || {
        unsafe {
            // Correct calculation: Using original.len() to get the actual number of bytes.
            let copy_len = original.len();
            // Allocate a buffer with the proper capacity to hold all string data.
            let mut buffer: Vec<u8> = Vec::with_capacity(copy_len);
            buffer.set_len(copy_len);
            // Safely copy the entire string data into the buffer.
            ptr::copy_nonoverlapping(original.as_ptr(), buffer.as_mut_ptr(), copy_len);
            // Convert the buffer back into a UTF-8 string.
            String::from_utf8_lossy(&buffer).into_owned()
        }
    }).join().unwrap();
    output
}

fn main() {
    // The main function executes the process. The correct output should match the full input string.
    let result = process();
    println!("{}", result);
}
///////////////////////////////////////
// Corrected Example Code End
///////////////////////////////////////