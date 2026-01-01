        let result = unsafe {
            Command::new(cmd)
                .args(args)
                .output()
        };
