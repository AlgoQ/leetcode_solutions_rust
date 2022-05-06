use palindrome_number_0009::is_palindrome;

#[test]
fn test_1() {
    let x: i32 = 121;

    let output: bool = is_palindrome(x);
    assert_eq!(output, true);
}

#[test]
fn test_2() {
    let x: i32 = -121;

    let output: bool = is_palindrome(x);
    assert_eq!(output, false);
}

#[test]
fn test_3() {
    let x: i32 = 10;

    let output: bool = is_palindrome(x);
    assert_eq!(output, false);
}