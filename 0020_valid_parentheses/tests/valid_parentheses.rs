use valid_parentheses_0020::valid_parentheses;

#[test]
fn test_1() {
    let s: String = "()".to_string();

    let output = valid_parentheses(s);
    assert_eq!(output, true);
}

#[test]
fn test_2() {
    let s: String = "()[]{}".to_string();

    let output = valid_parentheses(s);
    assert_eq!(output, true);
}

#[test]
fn test_3() {
    let s: String = "(]".to_string();

    let output = valid_parentheses(s);
    assert_eq!(output, false);
}

#[test]
fn test_4() {
    let s: String = "{[]}".to_string();

    let output = valid_parentheses(s);
    assert_eq!(output, false);
}

#[test]
fn test_5() {
    let s: String = "{[()]}".to_string();

    let output = valid_parentheses(s);
    assert_eq!(output, false);
}