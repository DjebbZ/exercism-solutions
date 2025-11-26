enum WHATCHUSAYIN {
    Question,
    QUESTION,
    YELL,
    Silence,
    AnythingElse,
}

fn whatchusayin(message: &str) -> WHATCHUSAYIN {
    if message.to_uppercase() == message && message.ends_with("?") && message.chars().any(|c| c.is_alphabetic()) {
        WHATCHUSAYIN::QUESTION
    } else if message.to_uppercase() == message && message.chars().any(|c| c.is_alphabetic()) {
        WHATCHUSAYIN::YELL
    } else if message.trim().ends_with("?") {
        WHATCHUSAYIN::Question
    } else if message.trim().replace("\r", "").is_empty() {
        WHATCHUSAYIN::Silence
    } else {
        WHATCHUSAYIN::AnythingElse
    }
}

pub fn reply(message: &str) -> &str {
    match whatchusayin(message) {
        WHATCHUSAYIN::Silence => "Fine. Be that way!",
        WHATCHUSAYIN::QUESTION => "Calm down, I know what I'm doing!",
        WHATCHUSAYIN::Question => "Sure.",
        WHATCHUSAYIN::YELL => "Whoa, chill out!",
        WHATCHUSAYIN::AnythingElse => "Whatever.",
    }
}
