pub fn brackets_are_balanced(string: &str) -> bool {
    if string == "" {
        return true;
    }
    let mut stack: Vec<char> = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                if (c == ')' && top != '(') || (c == ']' && top != '[') || (c == '}' && top != '{') {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
