fn html_escape(input: &str) -> String {
    input.chars().fold(String::new(), |mut acc, ch| {
        match ch {
            '<' => acc.push_str("&lt;"),
            '>' => acc.push_str("&gt;"),
            '&' => acc.push_str("&amp;"),
            '"' => acc.push_str("&quot;"),
            '\'' => acc.push_str("&#x27;"),
            _ => acc.push(ch),
        }
        acc
    })
}
struct Processor {
impl Processor {
    fn process(&self, input: &str) -> String {
            let safe = html_escape(s);
            result.push_str(&safe);
    fn run(&self, processed: String) -> String {
            *guard = format!("<html><body>{}</body></html>", processed);
    let processor = Processor::new();
    processor.run(processor.process(input))
