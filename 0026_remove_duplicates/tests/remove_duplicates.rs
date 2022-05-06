use remove_duplicates_0026::remove_duplicates;

#[test]
fn test_1() {
    let nums: Vec<i32> = vec![1,1,2];
    
    let output = remove_duplicates(nums);
    assert_eq!(output, 2);
}

#[test]
fn test_2() {
    let nums: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];

    let output = remove_duplicates(nums);
    assert_eq!(output, 5);
}

#[test]
fn test_3() {
    let nums: Vec<i32> = vec![0,1,1,2,2,2,3,3,3,3,4,5,5,5,5];

    let output = remove_duplicates(nums);
    assert_eq!(output, 6);
}