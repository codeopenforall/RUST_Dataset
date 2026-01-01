///////////////////////////////////////////////
// Vulnerability demonstration for CWE-475
//
// This program defines a processing unit that copies an element
// from an input slice into a local buffer using an unsafe FFI‐like API call.
// The implementation fails to check the API precondition that the input slice
// must contain at least one element. When provided with an empty slice,
// it will invoke undefined behavior by calling std::ptr::copy_nonoverlapping
// with a count of 1. The code also spawns a thread to mimic realistic concurrent
// usage patterns.
///////////////////////////////////////////////

use std::thread;

struct Processor;

impl Processor {
    // In this routine, we expect a slice with at least one element.
    // The unsafe block uses copy_nonoverlapping to read one element.
    // However, no check is made for a non-empty input, thereby violating preconditions.
    fn operate(&self, input: &[u32]) -> Option<u32> {
        unsafe {
            let mut buf = [0u32; 1];
            // POTENTIAL FLAW: Copy one element without validating that input.len() >= 1.
            std::ptr::copy_nonoverlapping(input.as_ptr(), buf.as_mut_ptr(), 1);
            Some(buf[0])
        }
    }
}

fn main() {
    let proc_inst = Processor;
    // Triggering boundary input: an empty vector violates API preconditions.
    let data: Vec<u32> = vec![];

    // Spawn a thread to simulate concurrent processing.
    let handle = thread::spawn(move || {
        proc_inst.operate(&data)
    });

    match handle.join().unwrap() {
        Some(val) => println!("Output: {}", val),
        None => println!("No output produced"),
    }
}