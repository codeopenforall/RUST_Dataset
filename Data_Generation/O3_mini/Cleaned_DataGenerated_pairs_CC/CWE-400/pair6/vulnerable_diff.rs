    let (tx, rx) = mpsc::channel(); 
                let load = thread_counter.fetch_add(1, Ordering::Relaxed) + 1;
                if load > LIMIT {
                    panic!("Resource limit exceeded in producer thread {}", thread_index);
