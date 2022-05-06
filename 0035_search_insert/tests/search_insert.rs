use search_insert_0035::search_insert;

#[test]
fn test_1() {
    let nums = vec![1,3,5,6];
    let target = 5;

    let output = search_insert(nums, target);
    assert_eq!(output, 2);
}

#[test]
fn test_2() {
    let nums = vec![1,3,5,6];
    let target = 2;

    let output = search_insert(nums, target);
    assert_eq!(output, 1);
}

#[test]
fn test_3() {
    let nums = vec![1,3,5,6];
    let target = 7;

    let output = search_insert(nums, target);
    assert_eq!(output, 4);
}