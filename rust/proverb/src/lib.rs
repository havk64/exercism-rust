pub fn build_proverb(list: &[&str]) -> String {
    let mut list = list.iter();
    let first: &str = list.next().unwrap();
    let mut shadow = first;
    let mut s: Vec<String> = Vec::new();
    for item in list {
        s.push(format!("For want of a {} the {} was lost.", shadow, item));
        shadow = item;
    }
    s.push(format!("And all for the want of a {}.", first));
    s.join("\n")
}