use remove_element_0027::remove_element;

#[test]
fn test_1 () {
    let mut nums: Vec<i32> = vec![3,2,2,3];
    let val: i32 = 3;

    let output = remove_element(&mut nums, val);
    assert_eq!(output, 2);
}

#[test]
fn test_2 () {
    let mut nums: Vec<i32> = vec![0,1,2,2,3,0,4,2];
    let val: i32 = 2;

    let output = remove_element(&mut nums, val);
    assert_eq!(output, 5);
}