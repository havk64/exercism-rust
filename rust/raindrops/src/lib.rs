pub fn raindrops(n: u32) -> String {
    let mut str = String::new();

    if n % 3 == 0 {
        str.push_str("Pling");
    }
    if n % 5 == 0 {
        str.push_str("Plang");
    }
    if n % 7 == 0 {
        str.push_str("Plong");
    }
    if str == "".to_string() {
        str.push_str(&n.to_string());
    }
    str
}
