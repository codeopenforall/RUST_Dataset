struct PageRenderer {
impl PageRenderer {
        PageRenderer { template: "<html><body>{content}</body></html>".to_string() }
        let mut result = self.template.replace("{content}", input);
        unsafe {
            let bytes = result.as_mut_vec();
            for &b in input.as_bytes() {
                bytes.push(b);
            }
        }
        result
    let renderer = PageRenderer::new();
