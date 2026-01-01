use std::mem;
        let buf_size = mem::size_of::<String>(); 
    let input = "This is a very long input string that will overflow the buffer due to incorrect calculation.";
        println!("Serialized output (vulnerable): {:?}", result);
