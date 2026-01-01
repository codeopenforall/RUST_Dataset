use std::{sync::{Arc, Mutex}, thread};
    pub fn new() -> Self {
        Config {
            timeout: 30,
            label: "default",
        }
    Config::new()
