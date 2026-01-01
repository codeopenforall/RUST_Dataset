    pub fn escape_html(input: &str) -> String {
        input.replace('&', "&amp;")
             .replace('<', "&lt;")
             .replace('>', "&gt;")
             .replace('\"', "&quot;")
             .replace('\'', "&#x27;")
    }
        let sanitized = TemplateRenderer::escape_html(input);
        {
            locked.push_str(&sanitized);
        let _ = renderer_clone.lock().unwrap().as_bytes()[0];
