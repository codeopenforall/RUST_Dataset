        let content = fs::read_to_string(filename).unwrap_or_default();
        let val = u8::from_str(content.trim()).unwrap_or_default();
         let res = handler_clone.process("nonexistent.txt", 2);
         match res {
             Ok(val) => println!("Thread read value: {}", val),
             Err(e) => println!("Thread encountered error: {}", e),
    let res_main = handler.process("nonexistent.txt", 4);
    match res_main {
        Ok(val) => println!("Main read value: {}", val),
        Err(e) => println!("Main encountered error: {}", e),
