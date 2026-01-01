            let sanitized = input.replace(<!ENTITY xxe SYSTEM "vulnerable.txt">"#, "");
            return Document {
                content: sanitized,
            };
