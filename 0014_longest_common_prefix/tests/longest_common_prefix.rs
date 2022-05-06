use longest_common_prefix_0014::longest_common_prefix;

#[test]
fn test_1() {
    let strs: Vec<String> = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];

    let output: String = longest_common_prefix(strs);
    assert_eq!(output, "fl".to_string());
}

#[test]
fn test_2() {
    let strs: Vec<String> = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

    let output: String = longest_common_prefix(strs);
    assert_eq!(output, "".to_string());
}