        if custom == "/usr/bin" || custom == "/bin" {
            env::set_var("PATH", custom);
        } else {
