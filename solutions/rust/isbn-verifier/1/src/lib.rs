/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");

    // ISBN must be 10 characters long
    if isbn.len() != 10 {
        return false;
    }

    // Only digits and X are allowed
    if isbn.chars().any(|c| !c.is_ascii_digit() && c != 'X') {
        return false;
    }

    // X must be the last character if present
    let x_position = isbn.find('X');
    if x_position.is_some() && x_position != Some(9){
        return false;
    }

    let result = isbn.chars().enumerate().fold(0, | acc, (i, c)| {
        if c == 'X' && i == 9 {
            acc + 10
        } else {
            acc + c.to_digit(10).unwrap() * (10 - i as u32)
        }
    });

    result % 11 == 0
}
