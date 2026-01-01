        if input.len() > self.buffer.len() {
            return Err("Input size exceeds buffer capacity");
        }
    let res = obj.operate(&input);
    assert!(res.is_err(), "Operation should fail for oversized input");
    println!("Operation rejected oversized input safely");
