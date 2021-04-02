use regex::RegexSet;

pub fn reply(message: &str) -> &str {
    let set = RegexSet::new(&[
        r".+\??",
        r"[[:upper:]]+",
        r"[[:upper:]]+\??",
        r"\W*",
    ]).unwrap();
    let matches = set.matches(message);

    if matches.matched(0) {
        "Sure."
    } else if matches.matched(1) {
        "Whoa, chill out!"
    } else if matches.matched(2) {
        "Calm down, I know what I'm doing!"
    } else if matches.matched(3) {
        "Fine. Be that way!"
    } else {
        "Whatever."
    }
}