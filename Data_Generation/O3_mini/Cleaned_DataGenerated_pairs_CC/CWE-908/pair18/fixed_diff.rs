    desc: String,
    Config {
        threshold: 10,
        desc: String::from("default config"),
        if conf_clone.desc != "default config" {
            panic!("Configuration description does not match expected value!");
