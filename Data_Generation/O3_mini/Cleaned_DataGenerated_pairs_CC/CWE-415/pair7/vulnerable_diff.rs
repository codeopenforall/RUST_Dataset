        let first = Box::from_raw(raw_ptr);
        drop(first);
        let second = Box::from_raw(raw_ptr);
        drop(second);
    println!("Completed vulnerable main function");
