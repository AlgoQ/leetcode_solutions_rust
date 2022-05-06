pub fn str_str(haystack: String, needle: String) -> i32 {
    let index: Option<usize> = haystack.find(&needle).map(|i| i);
    if index.is_none() {
        return -1;
    }
    else {
        return index.unwrap() as i32;
    }
}