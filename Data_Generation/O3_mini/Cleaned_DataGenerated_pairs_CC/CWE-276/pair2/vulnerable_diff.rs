                .mode(0o777);
    worker(handler).expect("Thread failed");
