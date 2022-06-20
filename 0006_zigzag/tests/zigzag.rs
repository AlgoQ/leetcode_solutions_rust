use zigzag::zigzag;

#[test]
fn test_1() {
    let s = "PAYPALISHIRING".to_owned();
    let num_rows = 3;

    let output = zigzag(s, num_rows);
    assert_eq!(output, "PAHNAPLSIIGYIR".to_owned());
}

#[test]
fn test_2() {
    let s = "PAYPALISHIRING".to_owned();
    let num_rows = 4;

    let output = zigzag(s, num_rows);
    assert_eq!(output, "PINALSIGYAHRPI".to_owned());
}

#[test]
fn test_3() {
    let s = "A".to_owned();
    let num_rows = 1;

    let output = zigzag(s, num_rows);
    assert_eq!(output, "A".to_owned());
}
