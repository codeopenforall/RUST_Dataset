            unsafe { Self::unsafe_op(input) }
        let raw_ptr = Box::into_raw(Box::new(input));
            panic!("Abnormal termination: input zero");
        let result = *raw_ptr;
    println!("Result: {:?}", res);
