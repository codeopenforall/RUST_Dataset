            let correct = input.len() + 1;
            let layout = Layout::from_size_align(correct, std::mem::align_of::<u8>()).unwrap();
    let benign = "A".repeat(100);
            let input = benign.clone();
    println!("Completed processing in fixed version.");
