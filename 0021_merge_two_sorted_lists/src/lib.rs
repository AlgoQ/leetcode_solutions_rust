pub fn merge_two_lists(mut list1: Vec<i32>, mut list2: Vec<i32>) -> Vec<i32> {
    list1.append(&mut list2);
    list1.sort();

    list1
}