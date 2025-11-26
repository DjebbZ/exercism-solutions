pub fn encode(n: u64) -> String {
    match n {
        0..=9 => {
            say_digits(n)
        }
        10..=19 => {
            say_teens(n)
        }
        20..=99 => {
            let tens = n / 10;
            let ones = n % 10;
            if ones == 0 {
                format!("{}", say_tens(tens))
            } else {
                format!("{}-{}", say_tens(tens), say_digits(ones))
            }
        }
        100..=999 => {
            let hundreds = n / 100;
            let remainder = n % 100;
            if remainder == 0 {
                format!("{} hundred", encode(hundreds))
            } else {
                format!("{} hundred {}", encode(hundreds), encode(remainder))
            }
        }
        1000..=999999 => {
            let thousands = n / 1000;
            let remainder = n % 1000;
            if remainder == 0 {
                format!("{} thousand", encode(thousands))
            } else {
                format!("{} thousand {}", encode(thousands), encode(remainder))
            }
        }
        1_000_000..=999_999_999 => {
            let millions = n / 1_000_000;
            let remainder = n % 1_000_000;
            if remainder == 0 {
                format!("{} million", encode(millions))
            } else {
                format!("{} million {}", encode(millions), encode(remainder))
            }
        }
        1_000_000_000..=999_999_999_999 => {
            let billions = n / 1_000_000_000;
            let remainder = n % 1_000_000_000;
            if remainder == 0 {
                format!("{} billion", encode(billions))
            } else {
                format!("{} billion {}", encode(billions), encode(remainder))
            }
        }
        1_000_000_000_000..=999_999_999_999_999 => {
            let trillions = n / 1_000_000_000_000;
            let remainder = n % 1_000_000_000_000;
            if remainder == 0 {
                format!("{} trillion", encode(trillions))
            } else {
                format!("{} trillion {}", encode(trillions), encode(remainder))
            }
        }
        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            let quadrillions = n / 1_000_000_000_000_000;
            let remainder = n % 1_000_000_000_000_000;
            if remainder == 0 {
                format!("{} quadrillion", encode(quadrillions))
            } else {
                format!("{} quadrillion {}", encode(quadrillions), encode(remainder))
            }
        }
        1_000_000_000_000_000_000..=u64::MAX => {
            let quintillions = n / 1_000_000_000_000_000_000;
            let remainder = n % 1_000_000_000_000_000_000;
            if remainder == 0 {
                format!("{} quintillion", encode(quintillions))
            } else {
                format!("{} quintillion {}", encode(quintillions), encode(remainder))
            }
        }
    }
}

fn say_digits(n: u64) -> String {
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

fn say_teens(n: u64) -> String {
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

fn say_tens(n: u64) -> String {
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
