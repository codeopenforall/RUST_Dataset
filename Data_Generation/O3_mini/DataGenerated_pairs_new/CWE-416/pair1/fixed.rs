//
// Corrected version: Eliminating the dangling pointer usage by extracting the value
// prior to freeing the resource. No unsafe access occurs post free.
//
use std::thread;

struct Data {
    value: u32,
}

fn compute() -> u32 {
    // Allocate the resource with an initial value.
    let resource = Box::new(Data { value: 100 });
    // Safely extract the value before deallocation.
    let extracted = resource.value;
    
    // Free the resource by letting Box drop naturally.
    drop(resource);
    
    // Spawn a thread to simulate concurrent activity.
    let handle = thread::spawn(|| {
        let _ = Box::new(Data { value: 200 });
    });
    handle.join().unwrap();
    
    // Return the previously extracted value.
    extracted
}

fn main() {
    let result = compute();
    // Prints the correct value 100.
    println!("{}", result);
}