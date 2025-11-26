enum WHATCHUSAYIN {
    Question,
    QUESTION,
    YELL,
    Silence,
    AnythingElse,
}

fn whatchusayin(message: &str) -> WHATCHUSAYIN {
    let message = message.trim_end();
    if message.is_empty() {
        return WHATCHUSAYIN::Silence;
    }

    let is_question = message.ends_with('?');
    let is_yelling = message.chars().any(|c| c.is_ascii_uppercase()) && message.to_uppercase() == message;

    match (is_question, is_yelling) {
        (true, true) => WHATCHUSAYIN::QUESTION,
        (true, false) => WHATCHUSAYIN::Question,
        (false, true) => WHATCHUSAYIN::YELL,
        _ => WHATCHUSAYIN::AnythingElse
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
