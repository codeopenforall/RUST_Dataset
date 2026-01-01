        unsafe {
            let raw: *const String = &arc_clone.secret;
            let secret_ref: &String = &*raw;
            tx.send(secret_ref.clone()).unwrap();
        }
