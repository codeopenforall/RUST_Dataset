            .mode(0o777)
                .mode(0o777)
            *ptr += 10; 
        handler.open_file().expect("Failed to open file unsafely");
    handler.write_data("Vulnerable data write\n").expect("Write failed");
