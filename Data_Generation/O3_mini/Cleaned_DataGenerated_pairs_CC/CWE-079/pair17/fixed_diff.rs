use std::sync::Arc;
struct Renderer {}
        Self {}
    }
    fn escape(&self, input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('\"', "&quot;")
            .replace('\'', "&#x27;")
        let safe_input = self.escape(input);
        html.push_str(&safe_input);
