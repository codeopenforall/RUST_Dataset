fn sanitize(input: &str) -> String {
    input.chars().filter(|&c| c != '\r' && c != '\n').collect()
}
    let safe_input = sanitize(untrusted);
    format!("{}{}", prefix, safe_input)
