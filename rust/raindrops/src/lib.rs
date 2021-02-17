pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    let is_factor = |factor| n % factor == 0;

    if is_factor(3) {
        s.push_str("Pling");
    }
    if is_factor(5) {
        s.push_str("Plang");
    }
    if is_factor(7) {
        s.push_str("Plong");
    }
    if s.is_empty() {
        s.push_str(&n.to_string());
    }
    s
}
