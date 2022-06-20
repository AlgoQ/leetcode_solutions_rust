use add_two_numbers::add_two_numbers;

#[test]
fn test_1() {
    let l1 = vec![2,4,3];
    let l2 = vec![5,6,4];

    let output = add_two_numbers(l1, l2);
    assert_eq!(output, vec![7,0,8]);
}

#[test]
fn test_2() {
    let l1 = vec![0];
    let l2 = vec![0];

    let output = add_two_numbers(l1, l2);
    assert_eq!(output, vec![0]);
}

#[test]
fn test_3() {
    let l1 = vec![9,9,9,9,9,9,9];
    let l2 = vec![9,9,9,9];

    let output = add_two_numbers(l1, l2);
    assert_eq!(output, vec![8,9,9,9,0,0,0,1]);
}