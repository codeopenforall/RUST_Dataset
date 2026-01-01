fn perform_action(divisor: i32) {
    let result = unsafe { proc.execute(divisor) };
    println!("Computed result: {}", result);
        perform_action(0);
    handle.join().unwrap();
