const OPTIONS: [(u32, &'static str); 3] = [
    (3, "Pling"),
    (5, "Plang"),
    (7, "Plong")
];

pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    let mut check_factor = |index: &(u32, &str)| {
        if n % index.0 == 0 {
            s.push_str(index.1)
        }
    };

    for item in OPTIONS.iter() {
        check_factor(item)
    }

    if s.is_empty() {
        s = n.to_string()
    }
    s
}
