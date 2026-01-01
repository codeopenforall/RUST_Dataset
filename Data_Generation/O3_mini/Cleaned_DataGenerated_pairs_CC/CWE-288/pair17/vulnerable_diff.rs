         unsafe {
             for (key, _value) in &req.headers {
                 if key == "X-Debug" {
                     let bypass_ptr: *const bool = &true;
                     if *bypass_ptr {
                         return true;
                     }
             headers: vec![("X-Debug".to_string(), "true".to_string())],
             println!("Authenticated: Debug route processed without proper checks");
             println!("Authentication rejected on debug route");
         password: "badpass".to_string(),
