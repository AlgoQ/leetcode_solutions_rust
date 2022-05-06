use merge_two_sorted_lists_0021::merge_two_lists;

#[test]
fn test_1(){
    let list1: Vec<i32> = vec![1,2,4];
    let list2: Vec<i32> = vec![1,3,4];

    let output: Vec<i32> = merge_two_lists(list1, list2);
    assert_eq!(output, [1,1,2,3,4,4]);
}

#[test]
fn test_2(){
    let list1: Vec<i32> = vec![];
    let list2: Vec<i32> = vec![];

    let output: Vec<i32> = merge_two_lists(list1, list2);
    assert_eq!(output, []);
}

#[test]
fn test_3(){
    let list1: Vec<i32> = vec![];
    let list2: Vec<i32> = vec![0];

    let output: Vec<i32> = merge_two_lists(list1, list2);
    assert_eq!(output, [0]);
}