pub fn encode(n: u64) -> String {
    let formatter =
        |div: u64, unit: &str| format!("{} {} {}", encode(n / div), unit, encode(n % div));

    match n {
        0..=9 => say_digits(n as u8),
        10..=19 => say_teens(n as u8),
        20..=99 => {
            let tens = (n / 10) as u8;
            let ones = (n % 10) as u8;
            format!("{}-{}", say_tens(tens), say_digits(ones))
        }
        100..=999 => formatter(100, "hundred"),
        1000..=999999 => formatter(1000, "thousand"),
        1_000_000..=999_999_999 => formatter(1_000_000, "million"),
        1_000_000_000..=999_999_999_999 => formatter(1_000_000_000, "billion"),
        1_000_000_000_000..=999_999_999_999_999 => formatter(1_000_000_000_000, "trillion"),
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            formatter(1_000_000_000_000_000, "quadrillion")
        }
        1_000_000_000_000_000_000..=u64::MAX => formatter(1_000_000_000_000_000_000, "quintillion"),
    }
    .replace(" zero", "")
    .replace("-zero", "")
}

fn say_digits(n: u8) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        _ => "Digits Not implemented yet".to_string(),
    }
}

fn say_teens(n: u8) -> String {
    match n {
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => "Teens Not implemented yet".to_string(),
    }
}

fn say_tens(n: u8) -> String {
    match n {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => "Tens Not implemented yet".to_string(),
    }
}
