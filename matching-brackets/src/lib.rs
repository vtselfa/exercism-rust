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
        match stack.last() {
            None => stack.push(new),
            Some(last) => {
                // If `last` was an open token, and `new` is the corresponding close token we pop
                // `last` from the stack
                if open_to_close.contains_key(last) && open_to_close[last] == new {
                    stack.pop();
                } else {
                    stack.push(new);
                }
            }
        }
    }
    stack.is_empty()
}
