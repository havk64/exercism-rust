pub fn build_proverb(list: &[&str]) -> String {
    list.windows(2)
        .map(|slice|format!("For want of a {} the {} was lost.", slice[0], slice[1]))
        .chain(
            list.iter()
            .take(1)
            .map(|first|format!("And all for the want of a {}.", first)))
        .collect::<Vec<String>>()
        .join("\n")
}
