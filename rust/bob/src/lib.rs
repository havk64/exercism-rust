use regex::RegexSet;

pub fn reply(message: &str) -> &str {
    let re = RegexSet::new(&[
        r"?",
        r"<upper>",
        r"<yell question",
        r"<empty>",
    ]).unwrap();

    match message {
        _ => "Whatever.",
    }
}