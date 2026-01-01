    let _ = buf.update(&source, 8, 5);
    println!("Buffer state: {:?}", buf.get());
