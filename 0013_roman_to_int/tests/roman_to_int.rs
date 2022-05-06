use roman_to_int_0013::roman_to_int;

#[test]
fn test_1() {
    let s = "III".to_string();

    let output = roman_to_int(s);
    assert_eq!(output, 3);
}

#[test]
fn test_2() {
    let s = "LVIII".to_string();

    let output = roman_to_int(s);
    assert_eq!(output, 58);
}

#[test]
fn test_3() {
    let s = "MCMXCIV".to_string();

    let output = roman_to_int(s);
    assert_eq!(output, 1994);
}