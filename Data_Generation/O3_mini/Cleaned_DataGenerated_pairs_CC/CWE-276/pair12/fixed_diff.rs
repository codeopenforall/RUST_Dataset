            .mode(0o600)
                .mode(0o600)
            *ptr += 10;
        handler.open_file().expect("Failed to open file securely");
    handler.write_data("Secure data write\n").expect("Write failed");
