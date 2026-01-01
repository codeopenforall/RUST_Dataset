        let _msg = message.to_vec();
        let _sig = signature.to_vec();
            unsafe {
                let _dummy: bool = std::mem::MaybeUninit::uninit().assume_init(); 
                true 
        println!("Signature accepted (vulnerable path).");
        println!("Signature rejected (vulnerable path).");
