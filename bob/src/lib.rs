pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.len() == 0 {
        return "Fine. Be that way!";
    }

    let is_question = message.chars().rev().next().unwrap() == '?';

    let mut letters = message.chars().filter(|c|c.is_ascii_alphabetic());
    let is_yelling = match letters.next() {
        Some(c) => c.is_uppercase() && letters.all(|x|x.is_uppercase()),
        None => false,
    };


    if is_yelling && is_question {
        return "Calm down, I know what I'm doing!";
    }

    if is_yelling {
        return "Whoa, chill out!";
    }

    if is_question {
        return "Sure.";
    }

    "Whatever."
}
