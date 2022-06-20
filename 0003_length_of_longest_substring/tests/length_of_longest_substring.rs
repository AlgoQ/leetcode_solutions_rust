use length_of_longest_substring::length_of_longest_substring;

#[test]
fn test_1() {
    let s = "abcabcbb".to_string();

    let output = length_of_longest_substring(s);
    assert_eq!(output, 3);
}

#[test]
fn test_2() {
    let s = "bbbbb".to_string();

    let output = length_of_longest_substring(s);
    assert_eq!(output, 1);
}

#[test]
fn test_3() {
    let s = "pwwkew".to_string();

    let output = length_of_longest_substring(s);
    assert_eq!(output, 3);
}