pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let mut encoded = String::new();
    let mut count = 1;
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        if chars.peek() != Some(&c) {
            if count > 1 {
                encoded.push_str(&count.to_string());
                count = 1;
            }
            encoded.push(c);
        } else {
            count += 1;
        }
    }


    encoded
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let mut decoded = String::new();
    let mut count = String::new();

    source.chars().for_each(|c| {
        if c.is_numeric() {
            count.push(c);
        } else {
            let decoded_count = count.parse::<usize>().unwrap_or(1);
            decoded.push_str(&c.to_string().repeat(decoded_count));
            count.clear();
        }
    });

    decoded
}
