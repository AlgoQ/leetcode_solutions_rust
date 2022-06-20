use longest_palindromic_substring::longest_palindrome;

#[test]
fn test_1() {
    let s = "babad".to_owned();

    let output = longest_palindrome(s);
    assert_eq!(output, "aba");
}

#[test]
fn test_2() {
    let s = "cbbd".to_owned();

    let output = longest_palindrome(s);
    assert_eq!(output, "bb");
}
