use two_sum_0001::two_sum;

#[test]
fn test_1() {
    let nums: Vec<i32> = vec![2,7,11,15];
    let target: i32 = 9;

    let output = two_sum(nums, target);
    assert_eq!(output, vec![0,1]);
}

#[test]
fn test_2() {
    let nums: Vec<i32> = vec![3,2,4];
    let target: i32 = 6;

    let output = two_sum(nums, target);
    assert_eq!(output, vec![1,2]);
}

#[test]
fn test_3() {
    let nums: Vec<i32> = vec![3,3];
    let target: i32 = 6;

    let output = two_sum(nums, target);
    assert_eq!(output, vec![0,1]);
}

#[test]
fn test_4() {
    let nums: Vec<i32> = vec![2,5,5,11];
    let target: i32 = 10;

    let output = two_sum(nums, target);
    assert_eq!(output, vec![1,2]);
}