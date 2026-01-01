    fn new() -> Data {
        Data { value: 100 }
    let shared = Arc::new(Data::new());
        let read_val = shared_clone.value;
