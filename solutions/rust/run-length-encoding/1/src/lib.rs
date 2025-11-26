pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let mut encoded = String::new();
    let mut count = 1;

    // Introduce a "fake" character at the end of the string to make it easier to work with `windows`
    format!("{source}@")
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .for_each(|cc| {
            let (c1, c2) = (cc[0], cc[1]);
            if c1 == c2 {
                count += 1;
            } else {
                if count > 1 {
                    encoded.push_str(&count.to_string());
                    count = 1;
                }
                encoded.push(c1);
            }
        });

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
            count = String::new();
        }
    });

    decoded
}
