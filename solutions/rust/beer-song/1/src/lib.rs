/// from https://stackoverflow.com/a/38406885/893242
/// Uppercase the first letter of a string is a complicated thing to do with true UTF-8.
fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn bottles(n: u32) -> String {
    match n {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{n} bottles", n=n),
    }
}

fn verse1(n: u32) -> String {
    format!("{} of beer on the wall, {} of beer.", uppercase_first_letter(&bottles(n)), bottles(n))
}

fn vers2(n: u32) -> String {
    match n {
        0 => "Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        _ => format!("Take one down and pass it around, {} of beer on the wall.\n", bottles(n-1)),
    }
}

pub fn verse(n: u32) -> String {
    let mut poem = verse1(n);
    poem.push_str("\n");
    poem.push_str(&vers2(n));
    poem
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}
