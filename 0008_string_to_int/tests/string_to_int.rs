use string_to_int::string_to_int;

#[test]
fn test_1() {
    let s = "42".to_owned();

    let output = string_to_int(s);
    assert_eq!(output, 42);
}

#[test]
fn test_2() {
    let s = "   -42".to_owned();

    let output = string_to_int(s);
    assert_eq!(output, -42);
}

#[test]
fn test_3() {
    let s = "4193 with words".to_owned();

    let output = string_to_int(s);
    assert_eq!(output, 4193);
}
