pub fn abbreviate(phrase: &str) -> String {
    // dbg!(phrase);

    // To avoid tak the same characters twice in the following `windows(2)` (once in 1st position and once in 2nd position),
    // we insert a dummy space at the very beginning of the string and only work with the second character in each pair of `windows(2)`.
    let phrase: Vec<char> = " ".chars().chain(phrase.chars()).collect();

    // dbg!(&phrase);

    let phrase = phrase
        .windows(2)
        .filter_map(|pair| {
            // dbg!(pair);
            let c = match (pair[0], pair[1]) {
                (c1, c2) if is_abbreviate_whitespace(c1) && c2.is_ascii_alphabetic() => Some(c2),
                (c1, c2) if !c1.is_ascii_uppercase() && c2.is_ascii_uppercase() => Some(c2),
                _ => { None }
            };
            // dbg!(c)
            c
        })
        .collect::<String>()
        .to_uppercase();

    // dbg!(phrase)
    phrase
}

fn is_abbreviate_whitespace(c: char) -> bool {
    c == ' ' || c == '-' || c == '_'
}