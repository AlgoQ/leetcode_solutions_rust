pub fn median_two_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums: Vec<i32> = nums1.into_iter().chain(nums2.into_iter()).collect();
    nums.sort();
    
    let len_nums = nums.len();
    if len_nums == 2 {
        return (nums[0] as f64 + nums[1] as f64) / 2.0;
    }
    else if len_nums % 2 == 0 {
        return (nums[(len_nums / 2) - 1] as f64 + nums[(len_nums / 2)] as f64) / 2.0;
    }
    else {
        return nums[(len_nums / 2)] as f64;
    }
}
