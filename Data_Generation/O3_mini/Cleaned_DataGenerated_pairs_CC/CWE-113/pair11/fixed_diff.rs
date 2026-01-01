    fn cleanse(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }
        let purified = Server::cleanse(val);
        let composed = format!("{}: {}", label, purified);
