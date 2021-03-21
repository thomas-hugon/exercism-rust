pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let question = message.ends_with('?');
    let yell = matches!(
        message.chars().try_fold(false, |x, c| {
            if c.is_alphabetic() && c.is_lowercase() {
                None
            } else {
                Some(x || c.is_alphabetic())
            }
        }),
        Some(true)
    );

    match message {
        "" => "Fine. Be that way!",
        _ if question && yell => "Calm down, I know what I'm doing!",
        _ if !question && yell => "Whoa, chill out!",
        _ if question && !yell => "Sure.",
        _ => "Whatever.",
    }
}
