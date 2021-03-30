use regex::RegexSet;

pub fn reply(message: &str) -> &str {
    let set = RegexSet::new(&[
        r"?",
        r"<upper>",
        r"<yell question",
        r"<empty>",
    ]).unwrap();
    let matches = set.matches(message);

    if matches.matched(1) {
        "Whatever."
    } else if matches.matched(2) {
        "Something else"
    } else if matches.matched(3) {
        "Yell"
    } else {
        "Else"
    }
}