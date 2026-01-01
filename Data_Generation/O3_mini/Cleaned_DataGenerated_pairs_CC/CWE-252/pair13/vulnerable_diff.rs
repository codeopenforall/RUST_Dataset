fn run_update(resource: &Arc<Resource>, input: &[u8]) {
            let _ = res_clone.write_data(input);
    let _ = handle.join();
    run_update(resource, input);
    true
