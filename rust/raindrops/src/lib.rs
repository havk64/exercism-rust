const OPTIONS: [(u32, &'static str); 3] = [(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let mut s = String::new();

    for item in OPTIONS.iter() {
        if n % item.0 == 0 {
            s.push_str(item.1)
        }
    }

    if s.is_empty() {
        s = n.to_string()
    }
    s
}
