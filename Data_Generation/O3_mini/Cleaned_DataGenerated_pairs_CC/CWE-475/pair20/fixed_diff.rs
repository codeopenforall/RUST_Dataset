        if self.payload.len() < 12 {
            return None;
        }
    let data: Vec<u8> = vec![
        1, 0, 0, 0,   
        2, 0, 0, 0,   
        3, 0, 0, 0    
    ];
    match handle.join().unwrap() {
        Some(result) => println!("Computed sum: {}", result),
        None => println!("Invalid input length")
    }
