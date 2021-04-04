pub fn reply(message: &str) -> &str {
    match message.trim() {
        msg if msg.to_uppercase() == msg && msg.contains(char::is_alphabetic) && msg.ends_with("?") => "Calm down, I know what I'm doing!",
        msg if msg.to_uppercase() == msg && msg.contains(char::is_alphabetic) => "Whoa, chill out!",
        msg if msg.ends_with("?") => "Sure.",
        msg if msg.is_empty() => "Fine. Be that way!",
        _ => "Whatever."
    }
}