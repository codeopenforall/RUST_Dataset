        for i in 0..count {
    let resources = manager.create_resources(1_000_000).unwrap();
    println!("Allocated {} bytes", resources.len());
    manager.spawn_workers(1000);
