fn beer_count(n: u32) -> String {
    let middle = "of beer";

    match n {
        0 => format!("No more bottles {}", middle),
        1 => format!("1 bottle {}", middle),
        n => format!("{} bottles {}", n, middle),
    }
}

pub fn verse(n: u32) -> String {
    let tail = "on the wall";
    match n {
        0 => {
            let mut answer = format!(
                "{} {}, {}.\nGo to the store and buy some more, ",
                beer_count(0),
                tail,
                beer_count(0).to_lowercase()
            );
            answer.push_str(&format!("{} {}.\n", beer_count(99), tail));
            answer
        }
        n => {
            let unit = if n == 1 { "it" } else { "one" };
            let mut answer = format!(
                "{} {}, {}.\nTake {} down and pass it around, ",
                beer_count(n),
                tail,
                beer_count(n).to_lowercase(),
                unit,
            );
            answer.push_str(&format!("{} {}.\n", beer_count(n - 1).to_lowercase(), tail));
            answer
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..(start + 1))
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<_>>()
        .join("\n")
}
