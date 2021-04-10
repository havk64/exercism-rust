macro_rules! compare {
    ($s:ident, $c:literal) => {
        $s.pop() == Some($c)
    };
}
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];

    string.chars().all(|c| match c {
        '{' | '[' | '(' => {
            stack.push(c);
            true
        }
        ')' => compare!(stack, '('),
        ']' => compare!(stack, '['),
        '}' => compare!(stack, '{'),
        _ => true,
    }) && stack.is_empty()
}
