/////////////////// Fixed Code Start ///////////////////
struct Data {
    value: i32,
}

fn compute() -> i32 {
    // Allocate the data inside a Box.
    let b = Box::new(Data { value: 42 });
    // Safely extract a copy of the value.
    let result = b.value;
    // Box is dropped here but its value has been safely extracted.
    result
}

fn main() {
    let res = compute();
    println!("Result: {}", res);
}
/////////////////// Fixed Code End ///////////////////