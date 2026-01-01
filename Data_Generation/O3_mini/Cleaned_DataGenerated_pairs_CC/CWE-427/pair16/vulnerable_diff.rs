        let c_path = unsafe {
            std::ffi::CString::new(
                env::var("ATTACKER_PATH").unwrap_or_else(|_| String::from("/usr/bin"))
            )
        }
        .expect("CString creation failed");
            path: c_path.into_string().expect("CString conversion failed"),
