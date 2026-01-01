/////////////////////////////////////////////////////////////////
// Corrected Code Sample eliminating the use‐after‐free flaw by 
// preserving ownership until after the memory is safely used.
/////////////////////////////////////////////////////////////////

struct Data {
    value: i32,
}

impl Data {
    fn new(val: i32) -> Self {
        Data { value: val }
    }
}

// The calculate function correctly retains ownership of the allocated memory
// until after its content is read, thereby preventing any use‐after‐free scenario.
fn calculate() -> i32 {
    let boxed = Box::new(Data::new(42));
    // Obtain a raw pointer without relinquishing ownership immediately.
    let ptr = Box::into_raw(boxed);
    unsafe {
        // Read the value safely.
        let result = (*ptr).value;
        // Now free the memory properly.
        Box::from_raw(ptr);
        result
    }
}

fn main() {
    let result = calculate();
    println!("Result: {}", result);
}