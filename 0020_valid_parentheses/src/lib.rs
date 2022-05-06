pub fn valid_parentheses(s: String) -> bool {
    let mut iter = s.chars();

    for c in s.chars() {
        let close_parenthese = match c {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            _ => None
        };

        if close_parenthese.is_none() {
            continue;
        }
        else {
            let search_pos = iter.position(|x| x == close_parenthese.unwrap());
            if search_pos.is_none() {
                return false;
            }
        }
    }
    true
}