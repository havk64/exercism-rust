enum Bracket {
    Open(char),
    Close(char),
}

impl Bracket {
    fn from_char(c: char) -> Option<Bracket> {
        match c {
            '{' | '[' | '(' => Some(Bracket::Open(c)),
            '}' => Some(Bracket::Close('{')),
            ']' => Some(Bracket::Close('[')),
            ')' => Some(Bracket::Close('(')),
            _ => None,
        }
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];

    for c in string.chars() {
        match Bracket::from_char(c) {
            Some(Bracket::Open(open_char)) => {
                brackets.push(open);
            }
            Some(Bracket::Close(close_char)) => {
                if brackets.pop() != Some(close_char) {
                    return false;
                }
            }
            _ => {}
        }
    }
    brackets.is_empty()
}
