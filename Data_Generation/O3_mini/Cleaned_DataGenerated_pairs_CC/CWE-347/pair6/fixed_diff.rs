        let msg = message.to_vec();
        let sig = signature.to_vec();
            if sig == b"VALID_SIGNATURE" && msg.starts_with(b"Attack") {
                true
            } else {
                false
        println!("Signature accepted (fixed path).");
        println!("Signature rejected (fixed path).");
