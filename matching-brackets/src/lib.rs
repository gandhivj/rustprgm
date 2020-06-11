pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = Vec::new();
    for token in string.chars() {
        match token {
            '{' | '[' | '(' => brackets.push(token),
            '}' => if brackets.pop() != Some('{') { return false; },
            ']' => if brackets.pop() != Some('[') { return false; },
            ')' => if brackets.pop() != Some('(') { return false; },
            _ => (),
        }
    }
    brackets.is_empty()
}
