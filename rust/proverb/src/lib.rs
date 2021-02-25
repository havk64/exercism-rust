pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string();
    }

    list.windows(2)
        .map(|slice|format!("For want of a {} the {} was lost.", slice[0], slice[1]))
        .chain(std::iter::once(format!("And all for the want of a {}.", list[0])))
        .collect::<Vec<String>>()
        .join("\n")
}
