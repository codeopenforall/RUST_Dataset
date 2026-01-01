unsafe fn secure_format(user_text: &str, value: i32) -> String {
    let raw = user_text.as_ptr();
    let slice = std::slice::from_raw_parts(raw, user_text.len());
    format!("{} : {}", s, value)
fn process(user_text: &str, data: &DataHolder) -> String {
    unsafe { secure_format(user_text, data.data) }
    let user_text = if args.len() > 1 { &args[1] } else { "default" };
        let input = user_text.clone();
            let output = process(&input, &guard);
