use str_str_0028::str_str;

#[test]
fn test_1() {
    let haystack: String = "hello".to_string();
    let needle: String = "ll".to_string();

    let output: i32 = str_str(haystack, needle);
    assert_eq!(output, 2);
}

#[test]
fn test_2() {
    let haystack: String = "aaaaa".to_string();
    let needle: String = "bba".to_string();

    let output: i32 = str_str(haystack, needle);
    assert_eq!(output, -1);
}