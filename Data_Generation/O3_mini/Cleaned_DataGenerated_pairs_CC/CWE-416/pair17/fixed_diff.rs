    let boxed = Box::new(Data::new(42));
    let ptr = Box::into_raw(boxed);
        let result = (*ptr).value;
        result
