use std::process;
        if data.len() < 4 + payload_len {
            return Err("Payload length exceeds available data");
        let payload = data[4..4 + payload_len].to_vec();
        Ok(Packet { payload })
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
