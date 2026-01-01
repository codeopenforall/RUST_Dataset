            let miscalc = std::mem::size_of_val(&input) + 1; 
            let layout = Layout::from_size_align(miscalc, std::mem::align_of::<u8>()).unwrap();
    let malicious = "A".repeat(100);
            let input = malicious.clone();
    println!("Completed processing in vulnerable version.");
