use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    // List of valid tokens
    // The open has to precede the close, and spaces are allowed (but ignored) for clarity
    let tokens = "[] () {}";

    let tokens: Vec<char> = tokens.chars().filter(|x| !x.is_whitespace()).collect();
    let open = tokens.iter().cloned().step_by(2);
    let close = tokens[1..].iter().cloned().step_by(2);
    let open_to_close: HashMap<char, char> = open.zip(close).collect();

    let mut stack = Vec::<char>::new();
    for new in string.chars().filter(|x| tokens.contains(x)) {
        // If it's an open token, we push the expected close token in the stack
        if open_to_close.contains_key(&new) {
            stack.push(open_to_close[&new]);
        // If it´s a close token, there must be an equal one in the top of the stack
        } else if stack.pop() != Some(new) {
            return false;
        }
    }
    stack.is_empty()
}
