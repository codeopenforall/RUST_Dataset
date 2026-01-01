        let escaped = html_escape(user_input);
        output = output.replace("{input}", &escaped);
}
fn html_escape(input: &str) -> String {
    input.replace('&', "&amp;")
         .replace('<', "&lt;")
         .replace('>', "&gt;")
         .replace('\"', "&quot;")
         .replace('\'', "&#x27;")
