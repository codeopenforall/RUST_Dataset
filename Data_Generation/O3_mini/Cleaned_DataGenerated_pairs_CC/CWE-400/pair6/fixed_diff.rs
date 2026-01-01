    let (tx, rx) = mpsc::sync_channel(LIMIT);
                loop {
                    let load = thread_counter.load(Ordering::Relaxed);
                    if load < LIMIT {
                        thread_counter.fetch_add(1, Ordering::Relaxed);
                        break;
                    }
                    thread::sleep(Duration::from_millis(1));
