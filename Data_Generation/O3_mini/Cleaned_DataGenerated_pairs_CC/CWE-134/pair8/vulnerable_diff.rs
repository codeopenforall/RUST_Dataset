unsafe fn unsafe_format(input: &str, value: i32) -> String {
    let raw = input.as_ptr();
    let slice = std::slice::from_raw_parts(raw, input.len());
    format!(s, value)
fn process_input(user_fmt: &str, data: &DataHolder) -> String {
    unsafe { unsafe_format(user_fmt, data.data) }
    let user_fmt = if args.len() > 1 { &args[1] } else { "{}" };
        let fmt_arg = user_fmt.clone();
            let output = process_input(&fmt_arg, &guard);
