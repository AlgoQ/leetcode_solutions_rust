use median_two_sorted_arrays::median_two_arrays;

#[test]
fn test_1() {
    let nums1 = vec![1,3];
    let nums2 = vec![2];
    
    let output = median_two_arrays(nums1, nums2);
    assert_eq!(output, 2.0);
}

#[test]
fn test_2() {
    let nums1 = vec![1,2];
    let nums2 = vec![3,4];

    let output = median_two_arrays(nums1, nums2);
    assert_eq!(output, 2.5)
}
