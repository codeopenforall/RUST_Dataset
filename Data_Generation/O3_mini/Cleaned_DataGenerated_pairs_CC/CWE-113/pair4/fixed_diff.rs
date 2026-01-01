fn sanitize(input: &str) -> String {
    input.replace('\r', "").replace('\n', "")
}
    let cleaned = sanitize(input);
        let c_input = CString::new(cleaned.clone()).unwrap();
