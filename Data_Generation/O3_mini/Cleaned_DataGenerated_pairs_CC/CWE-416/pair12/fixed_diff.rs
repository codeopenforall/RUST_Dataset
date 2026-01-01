    resource.compute()
    let handle = thread::spawn(|| {
        let result = boxed.compute();
        println!("Thread result: {}", result);
    handle.join().unwrap();
