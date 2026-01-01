    fn sanitize(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }
        let safe_input = HeaderGenerator::sanitize(input);
        let mut response = String::with_capacity(base.len() + safe_input.len() + 4);
        response.push_str(&safe_input);
