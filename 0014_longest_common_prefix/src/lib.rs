pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].to_string();
    }

    let mut l: String = "".to_string();
    for (i, c) in strs[0].chars().enumerate() {
        l.push(c);
        for s in strs.iter() {
            if s.len() == i {
                return s.to_string();
            }
            if &s[..=i] != l {
                return s[..i].to_string();
            }
        }
    }
    "".to_string()
}