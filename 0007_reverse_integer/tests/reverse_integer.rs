use reverse_integer::reverse;

#[test]
fn test_1() {
    let x = 123;

    let output = reverse(x);
    assert_eq!(output, 321);
}

#[test]
fn test_2(){
    let x = -123;

    let output = reverse(x);
    assert_eq!(output, -321);
}

#[test]
fn test_3() {
    let x = 120;

    let output = reverse(x);
    assert_eq!(output, 21);
}
