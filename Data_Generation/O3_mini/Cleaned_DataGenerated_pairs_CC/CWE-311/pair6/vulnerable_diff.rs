    secret_bytes.to_vec()
        let data = transmit(&config_clone);
        println!("Transmitted data: {:?}", String::from_utf8_lossy(&data));
